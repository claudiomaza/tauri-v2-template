#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/// cm2labs Encrypted Token Vault — Cross-Platform
/// - AES-256-GCM con clave en RAM
/// - Escritura atómica (temp + rename)
/// - Store/Get/Clear via Tauri commands
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use tauri::Manager;

static VAULT_KEY: Mutex<Option<[u8; 32]>> = Mutex::new(None);

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

#[tauri::command]
fn store_token(token: String, app: tauri::AppHandle) -> Result<(), String> {
    let mut key_lock = VAULT_KEY.lock().map_err(|e| e.to_string())?;
    let key_bytes = if let Some(k) = key_lock.as_ref() {
        *k
    } else {
        let mut k = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut k);
        *key_lock = Some(k);
        k
    };
    drop(key_lock);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let mut nonce = [0u8; 12];
    rand::rngs::OsRng.fill_bytes(&mut nonce);
    let nonce_slice = Nonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(nonce_slice, token.as_bytes())
        .map_err(|e| format!("encrypt: {}", e))?;

    let mut payload = Vec::with_capacity(12 + ciphertext.len());
    payload.extend_from_slice(&nonce);
    payload.extend_from_slice(&ciphertext);

    let path = vault_path(&app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir: {}", e))?;
    }

    // Escritura atómica
    let temp_path = path.with_extension("enc.tmp");
    fs::write(&temp_path, &payload).map_err(|e| format!("write: {}", e))?;
    fs::rename(&temp_path, &path).map_err(|e| format!("rename: {}", e))?;
    Ok(())
}

#[tauri::command]
fn get_token(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let key_lock = VAULT_KEY.lock().map_err(|e| e.to_string())?;
    let key_bytes = match key_lock.as_ref() {
        Some(k) => *k,
        None => return Ok(None),
    };
    drop(key_lock);

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

#[tauri::command]
fn clear_vault(app: tauri::AppHandle) -> Result<(), String> {
    vault_clear(&app);
    let _ = fs::remove_file(vault_path(&app).with_extension("enc.tmp"));
    Ok(())
}

fn main() {
    let builder = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![store_token, get_token, clear_vault])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                vault_clear(&window.app_handle());
            }
        });

    // Solo agregar shell plugin en desktop (no disponible en Android/iOS)
    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_shell::init());

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}