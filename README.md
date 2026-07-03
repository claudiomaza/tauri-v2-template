# tauri-v2-template 🧩

Template multi-plataforma para **Tauri v2** — Windows, Linux, macOS, Android e iOS desde un mismo código.

## 🎮 Configuración de Build

En `.github/workflows/build.yml`, buscá esta sección y poné `'true'`/`'false'`:

```yaml
env:
  WINDOWS: "true"
  LINUX: "true"
  MACOS: "false"
  ANDROID: "false"
  IOS: "false"
```

| Plataforma | Binario | Runner CI |
|---|---|---|
| 🪟 Windows | `.exe` + NSIS installer | `windows-latest` |
| 🐧 Linux | `.deb` + AppImage | `ubuntu-latest` |
| 🍏 macOS | `.dmg` | `macos-latest` |
| 🤖 Android | `.apk` | `ubuntu-latest` (+ SDK) |
| 📱 iOS | `.app` (simulador) | `macos-latest` |

## 🔐 Seguridad: Persistent Session + Encrypted Vault

Arquitectura de doble capa:

### Session persistente via OS Keyring
Los tokens se guardan cifrados con **AES-256-GCM** en disco (`vault.enc`).
La clave AES se almacena en el **keyring nativo del SO** (Windows Credential Manager,
Linux Secret Service, macOS Keychain, Android KeyStore).

Esto significa que **la sesión persiste entre reinicios de la app y del SO**.
Solo se pierde si el usuario cierra sesión explícitamente.

### API frontend (JavaScript)
```javascript
// Guardar token (login)
await window.__TAURI__.invoke("store_token", { token: "..." })
// -> genera clave AES, la persiste en keyring, cifra token en vault.enc

// Leer token (auto-login si hay sesión)
await window.__TAURI__.invoke("get_token")
// -> recupera clave del keyring, descifra vault.enc

// Verificar si hay sesión activa
await window.__TAURI__.invoke("is_logged_in")
// -> true/false según si la clave existe en el keyring

// Cerrar sesión
await window.__TAURI__.invoke("clear_vault")
// -> borra la clave del keyring + vault.enc → token irrecuperable
```

## 🚀 Cómo usar
1. **Fork** este repositorio
2. Poné tu `index.html` en `ui/`
3. Ajustá `src-tauri/tauri.conf.json` (name, identifier, version)
4. Push a `main` — CI compila automáticamente

También podés disparar builds manuales desde **Actions → Run workflow** y elegir plataformas una por una.

## 📁 Estructura
```
.
├── .github/workflows/build.yml   ← CI/CD con toggles
├── ui/index.html                 ← TU app web (Tier 2 Compact)
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   ├── icons/
│   └── src/main.rs               ← Rust cross-platform + Vault + Keyring
└── package.json
```

## ⚡ Built with cm2labs

