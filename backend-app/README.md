# Taury CRM - Backend API

## Description
API backend Rust (Actix-web) pour la synchronisation des données.

## Prérequis
- Rust 1.70+
- PostgreSQL
- Docker (optionnel)

## Installation

### Option 1: Docker (Recommandé)
```bash
# Démarrer PostgreSQL
docker-compose up -d

# Build et run
cargo run
```

### Option 2: Local
```bash
# Installer PostgreSQL
# Configurer la base de données
# Modifier les variables d'environnement

cargo run
```

## Configuration
- Port : 8080
- Base de données : PostgreSQL
- CORS : Activé pour le frontend

## API Endpoints
- `GET /api/health` - Santé de l'API
- `POST /api/sync` - Synchronisation des données
- `GET /api/companies` - Liste des entreprises
- `GET /api/proposals` - Liste des propositions

## Variables d'environnement
```bash
DATABASE_URL=postgresql://user:password@localhost:5432/crm
PORT=8080
```

## Déploiement Raspberry Pi
1. Installer Rust sur le Pi
2. Cloner ce dossier
3. Configurer PostgreSQL
4. `cargo build --release`
5. `./target/release/crm-backend-api`

## Sidecar Python
Le dossier `sidecar-python/` contient l'API Python pour la génération de documents.
