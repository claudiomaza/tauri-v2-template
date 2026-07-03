# tauri-v2-template 🧩

Template multi-plataforma para **Tauri v2** — Windows, Linux, macOS, Android e iOS desde un mismo código.

## 🎮 Configuración de Build

En `.github/workflows/build.yml`, buscá esta sección y poné `'true'/`false'`:

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

## 🚀 Cómo usar
1. **Fork** este repositorio
2. Poné tu `index.html` en `ui/`
3. Ajustá `src-tauri/tauri.conf.json` (name, identifier, version)
4. Push a `main` — CI compila automáticamente

También podés disparar builds manuales desde **Actions → Run workflow** y elegir plataformas una por una.

## 🔐 Seguridad: Encrypted Token Vault

Bóveda de tokens AES-256-GCM con clave en RAM, escritura atómica y limpieza automática al cerrar.

### API frontend (JS)
```javascript
await window.__TAURI__.invoke("store_token", { token: "..." })
await window.__TAURI__.invoke("get_token")
await window.__TAURI__.invoke("clear_vault")
```

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
│   └── src/main.rs               ← Rust cross-platform
└── package.json
```

## ⚡ Built with cm2labs

