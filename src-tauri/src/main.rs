#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/// cm2labs Encrypted Token Vault + Persistent Session via OS Keyring
///
/// Arquitectura:
///   - Login → genera clave AES → la guarda en el keyring del SO → cifra token en vault.enc
///   - Inicio de app → lee clave del keyring → descifra vault.enc automáticamente
///   - Cerrar sesión → borra clave del keyring → vault.enc ilegible hasta nuevo login
///
/// La sesión persiste entre reinicios del SO. Solo se pierde si el usuario
/// cierra sesión explícitamente o si alguien borra la entrada del keyring.
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use tauri::Manager;
use keyring::Entry;

const KEYRING_SERVICE: &str = "cm2labs-vault";
const KEYRING_USER: &str = "aes-key";

// Clave AES en RAM durante la sesión activa (se pierde al cerrar app,
// pero el keyring la retiene para el próximo inicio)
static VAULT_KEY: Mutex<Option<[u8; 32]>> = Mutex::new(None);

// ─── Keyring helpers ─────────────────────────────────────

fn keyring_set(key_bytes: &[u8; 32]) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Keyring entry error: {e}"))?;
    entry
        .set_secret(key_bytes)
        .map_err(|e| format!("Keyring set error: {e}"))?;
    Ok(())
}

fn keyring_get() -> Result<Option<[u8; 32]>, String> {
    let entry = match Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        Ok(e) => e,
        Err(_) => return Ok(None),
    };
    let secret = match entry.get_secret() {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    if secret.len() != 32 {
        return Ok(None);
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&secret);
    Ok(Some(key))
}

fn keyring_delete() -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Keyring entry error: {e}"))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Keyring delete error: {e}"))
}

// ─── Vault helpers ───────────────────────────────────────

fn vault_path(app: &tauri::AppHandle) -> PathBuf {
    let mut dir = app.path().app_data_dir().expect("app data dir");
    dir.push("vault.enc");
    dir
}

fn vault_clear(app: &tauri::AppHandle) {
    if let Ok(mut k) = VAULT_KEY.lock() {
        *k = None;
    }
    let _ = fs::remove_file(vault_path(app));
}

fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

// ─── Tauri Commands ──────────────────────────────────────

#[tauri::command]
fn store_token(app: tauri::AppHandle, token: String) -> Result<(), String> {
    // 1. Generar o recuperar clave AES
    let key = {
        let mut key_lock = VAULT_KEY.lock().map_err(|e| e.to_string())?;
        match key_lock.as_ref() {
            Some(k) => *k,
            None => {
                let new_key = generate_key();
                *key_lock = Some(new_key);
                new_key
            }
        }
    };

    // 2. Persistir clave en el keyring del SO
    keyring_set(&key)?;

    // 3. Cifrar el token y escribirlo atómicamente
    let nonce_bytes = {
        let mut n = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut n);
        n
    };
    let aes_key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(aes_key);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), token.as_bytes())
        .map_err(|e| format!("Encrypt error: {e}"))?;

    let mut out = nonce_bytes.to_vec();
    out.extend_from_slice(&ciphertext);

    let path = vault_path(&app);
    let tmp_path = path.with_extension("enc.tmp");

    // Escritura atómica: temp file + rename
    fs::write(&tmp_path, &out).map_err(|e| format!("Write error: {e}"))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("Atomic rename error: {e}"))?;

    Ok(())
}

#[tauri::command]
fn get_token(app: tauri::AppHandle) -> Result<Option<String>, String> {
    // 1. Intentar cargar clave desde keyring si no está en RAM
    {
        let mut key_lock = VAULT_KEY.lock().map_err(|e| e.to_string())?;
        if key_lock.is_none() {
            if let Some(k) = keyring_get()? {
                *key_lock = Some(k);
            }
        }
    }

    // 2. Leer clave
    let key_bytes = {
        let key_lock = VAULT_KEY.lock().map_err(|e| e.to_string())?;
        match key_lock.as_ref() {
            Some(k) => *k,
            None => return Ok(None),
        }
    };

    // 3. Leer vault
    let data = match fs::read(vault_path(&app)) {
        Ok(d) => d,
        Err(_) => return Ok(None),
    };
    if data.len() < 12 {
        return Ok(None);
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext) {
        Ok(p) => Ok(Some(String::from_utf8(p).map_err(|e| e.to_string())?)),
        Err(_) => Ok(None),
    }
}

/// Borra la sesión: elimina la clave del keyring y el vault.
#[tauri::command]
fn clear_vault(app: tauri::AppHandle) -> Result<(), String> {
    let _ = keyring_delete();
    vault_clear(&app);
    let _ = fs::remove_file(vault_path(&app).with_extension("enc.tmp"));
    Ok(())
}

/// Indica si hay sesión activa (clave presente en keyring).
#[tauri::command]
fn is_logged_in() -> Result<bool, String> {
    match keyring_get() {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e),
    }
}

// ─── Main ────────────────────────────────────────────────

fn main() {
    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            store_token,
            get_token,
            clear_vault,
            is_logged_in
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Al cerrar la ventana: limpiamos solo la RAM, no el keyring.
                // La sesión persiste para el próximo inicio.
                if let Ok(mut k) = VAULT_KEY.lock() {
                    *k = None;
                }
            }
        });

    // Solo agregar shell plugin en desktop (no disponible en Android/iOS)
    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_shell::init());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}