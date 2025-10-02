# Taury CRM - Projet Séparé

## Structure du Projet

### 📱 Frontend App (`frontend-app/`)
Application desktop Tauri (Rust + React) pour la gestion commerciale.

**Compilation :**
```bash
cd frontend-app
npm install
npm run tauri build
```

### 🖥️ Backend App (`backend-app/`)
API backend Rust (Actix-web) pour la synchronisation des données.

**Déploiement sur Raspberry Pi :**
```bash
cd backend-app
cargo build --release
./target/release/crm-backend-api
```

## Architecture

- **Frontend** : Application desktop (Windows, macOS, Linux)
- **Backend** : API serveur (Raspberry Pi)
- **Base de données** : PostgreSQL
- **Synchronisation** : Bidirectionnelle entre local et serveur

## Documentation

- `DEPLOYMENT.md` - Guide complet de déploiement
- `frontend-app/README.md` - Documentation frontend
- `backend-app/README.md` - Documentation backend

## Fichiers

- `Export de tarification, 23 juil 2025.xlsx` - Données de tarification
