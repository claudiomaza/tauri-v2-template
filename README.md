# tauri-v2-template-win 🪟

Template base para aplicaciones Windows con **Tauri v2**.

## 📦 Requisitos
- Rust 1.77+
- Node.js 18+
- Windows (solo target Windows)

## 🚀 Cómo usar

1. **Fork** este repositorio
2. Crear carpeta `ui/` y poner tu `index.html` allí
3. Ajustar en `src-tauri/tauri.conf.json`:
   - `app.windows[0].title`
   - `app.identifier`
4. Pushear al fork — GitHub Actions compila automáticamente

## 📁 Estructura
```
.
├── .github/workflows/build.yml   # CI/CD → .exe
├── ui/
│   └── index.html                # TU app web acá
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   ├── icons/icon.ico
│   └── src/main.rs
└── package.json
```

## ⚡ Built with cm2labs
