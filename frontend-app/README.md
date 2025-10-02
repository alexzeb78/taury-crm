# Taury CRM - Frontend Application

## Description
Application desktop Tauri (Rust + React) pour la gestion commerciale.

## Prérequis
- Node.js 18+
- Rust (pour Tauri)
- npm ou yarn

## Installation
```bash
npm install
```

## Développement
```bash
npm run tauri dev
```

## Build
```bash
# Build pour la plateforme actuelle
npm run tauri build

# Build pour Windows
npm run tauri build -- --target x86_64-pc-windows-msvc

# Build pour Linux
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

## Structure
- `src/` - Code React/TypeScript
- `src-tauri/` - Code Rust (Tauri)
- `dist/` - Build frontend (généré)

## Configuration
- Base de données SQLite locale
- Synchronisation avec backend via API
- Génération de documents (Word, Excel)

## Backend
Le backend doit tourner sur un serveur séparé (Raspberry Pi).
URL par défaut : `http://localhost:8080`
