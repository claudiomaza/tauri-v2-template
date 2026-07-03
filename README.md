# tauri-v2-template-win 🪟

Template base para aplicaciones Windows con **Tauri v2**.

## 📦 Requisitos
- Rust 1.77+
- Node.js 18+
- Windows (target único)

## 🚀 Cómo usar
1. **Fork** este repositorio
2. Crear carpeta `ui/` y poner tu `index.html` allí
3. Ajustar en `src-tauri/tauri.conf.json`: `title` e `identifier`
4. Pushear a `main` — GitHub Actions compila automáticamente

## 📁 Estructura
```
.
├── .github/workflows/build.yml   # CI/CD → .exe
├── ui/index.html                 # TU app web acá
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   ├── icons/icon.ico
│   └── src/main.rs
└── package.json
```

## 🔐 Seguridad: Encrypted Token Vault

El template incluye una bóveda de tokens cifrados integrada en el backend Rust:

| Característica | Detalle |
|---|---|
| Cifrado | AES-256-GCM |
| Clave | En RAM, generada al primer `store_token` |
| Almacenamiento | `%APPDATA%/.../vault.enc` |
| Escritura | Atómica (temp + rename) — tolerante a crashes |
| Limpieza automática | Al cerrar ventana se borra clave y archivo |

`vault.enc` es ilegible sin la clave en RAM. Al reiniciar la PC la clave se pierde.

### API para el frontend
```javascript
await window.__TAURI__.invoke("store_token", { token: "..." })
await window.__TAURI__.invoke("get_token")
await window.__TAURI__.invoke("clear_vault")
```

## ⚡ Built with cm2labs

