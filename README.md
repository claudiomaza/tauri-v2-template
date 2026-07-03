# tauri-v2-cross-template 🧩

Template multi-plataforma para **Tauri v2** — compila a Windows, Linux y Android desde un mismo código.

## 📦 Targets

| Plataforma | Binario | Runner CI |
|---|---|---|
| 🪟 Windows | `.exe` + NSIS installer | `windows-latest` |
| 🐧 Linux | `.deb` + AppImage | `ubuntu-latest` |
| 📱 Android | `.apk` | `ubuntu-latest` (+ SDK) |

## 🚀 Cómo usar
1. **Fork** este repositorio
2. Poné tu `index.html` en `ui/`
3. Ajustá `src-tauri/tauri.conf.json` (name, identifier, version)
4. Push a `main` — CI compila automáticamente

## 🎮 Toggles de Build

Podés activar/desactivar plataformas de dos formas:

### Opción A — Editando el YML
En `.github/workflows/build.yml`, cambiá los valores:
```yaml
BUILD_WINDOWS: "true"
BUILD_LINUX: "true"
BUILD_ANDROID: "false"
```

### Opción B — Desde GitHub UI
Andá a **Actions → Build → Run workflow** y seleccioná qué compilar.

## 🔐 Seguridad: Encrypted Token Vault

Incluye bóveda de tokens AES-256-GCM con clave en RAM:

| Característica | Detalle |
|---|---|
| Cifrado | AES-256-GCM |
| Clave | En RAM, generada al primer `store_token` |
| Almacenamiento | `%APPDATA%` (Win), `~/.local/share` (Linux), app-specific (Android) |
| Escritura | Atómica (temp + rename) |
| Limpieza automática | Al cerrar ventana |

### API frontend (JS)
```javascript
await window.__TAURI__.invoke("store_token", { token: "..." })
await window.__TAURI__.invoke("get_token")
await window.__TAURI__.invoke("clear_vault")
```

## 📁 Estructura
```
.
├── .github/workflows/build.yml   ← CI/CD cross-platform
├── ui/index.html                 ← TU app web (Tier 2 Compact)
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   ├── icons/
│   └── src/main.rs               ← Rust cross-platform
└── package.json
```

## ⚡ Built with cm2labs

