# Guide de Déploiement - Taury CRM

## Architecture
- **Frontend** : Application Tauri (Rust + React) - Desktop
- **Backend** : API Rust (Actix-web) - Serveur (Raspberry Pi)
- **Base de données** : PostgreSQL - Serveur (Raspberry Pi)

## Déploiement Frontend

### 1. Compilation
```bash
cd frontend-app
npm install
npm run tauri build
```

### 2. Distribution
- **Windows** : `Taury CRM_0.1.0_x64_en-US.msi`
- **macOS** : `Taury CRM_0.1.0_aarch64.dmg`
- **Linux** : `taury-crm_0.1.0_amd64.deb`

## Déploiement Backend (Raspberry Pi)

### 1. Préparation du Pi
```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installer PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# Installer Docker (optionnel)
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
```

### 2. Configuration PostgreSQL
```bash
# Créer la base de données
sudo -u postgres psql
CREATE DATABASE crm;
CREATE USER crm_user WITH PASSWORD 'votre_mot_de_passe';
GRANT ALL PRIVILEGES ON DATABASE crm TO crm_user;
\q
```

### 3. Déploiement Backend
```bash
# Copier le dossier backend-app sur le Pi
scp -r backend-app/ pi@votre-pi-ip:/home/pi/

# Sur le Pi
cd backend-app
cargo build --release

# Lancer le backend
./target/release/crm-backend-api
```

### 4. Configuration Frontend
Modifier l'URL du backend dans le frontend :
```typescript
// Dans src/contexts/SyncContext.tsx
const serverUrl = 'http://votre-pi-ip:8080';
```

## Variables d'environnement

### Backend
```bash
DATABASE_URL=postgresql://crm_user:mot_de_passe@localhost:5432/crm
PORT=8080
```

### Frontend
```bash
VITE_API_URL=http://votre-pi-ip:8080
```

## Sécurité

### Firewall
```bash
# Sur le Raspberry Pi
sudo ufw allow 8080
sudo ufw enable
```

### SSL/TLS (Optionnel)
- Utiliser un reverse proxy (Nginx)
- Certificats Let's Encrypt
- HTTPS pour les communications

## Monitoring

### Logs Backend
```bash
# Voir les logs
journalctl -u crm-backend-api -f

# Logs Docker
docker logs crm-backend-api
```

### Base de données
```bash
# Connexion PostgreSQL
psql -h localhost -U crm_user -d crm

# Vérifier les tables
\dt
```

## Maintenance

### Sauvegarde
```bash
# Sauvegarde PostgreSQL
pg_dump -h localhost -U crm_user crm > backup_$(date +%Y%m%d).sql

# Restauration
psql -h localhost -U crm_user crm < backup_20241002.sql
```

### Mise à jour
```bash
# Backend
cd backend-app
git pull
cargo build --release
sudo systemctl restart crm-backend-api

# Frontend
cd frontend-app
git pull
npm run tauri build
```

## Dépannage

### Backend ne démarre pas
- Vérifier PostgreSQL
- Vérifier les variables d'environnement
- Vérifier les logs

### Frontend ne se connecte pas
- Vérifier l'URL du backend
- Vérifier le firewall
- Vérifier la connectivité réseau

### Synchronisation échoue
- Vérifier les timestamps
- Vérifier la base de données
- Vérifier les logs de sync
