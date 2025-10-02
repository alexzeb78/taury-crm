#!/bin/bash

# Script de déploiement Docker du backend sur Raspberry Pi
# Usage: ./deploy-raspberry-pi-docker.sh [IP_RASPBERRY_PI] [USERNAME]

set -e

# Configuration par défaut
DEFAULT_IP="192.168.1.100"
DEFAULT_USER="pi"
DEFAULT_PORT="22"

# Paramètres
RASPBERRY_IP=${1:-$DEFAULT_IP}
RASPBERRY_USER=${2:-$DEFAULT_USER}
RASPBERRY_PORT=${3:-$DEFAULT_PORT}

echo "🐳 Déploiement Docker du backend CRM sur Raspberry Pi"
echo "📍 IP: $RASPBERRY_IP"
echo "👤 Utilisateur: $RASPBERRY_USER"
echo "🔌 Port SSH: $RASPBERRY_PORT"
echo ""

# Vérifier que nous sommes dans le bon répertoire
if [ ! -d "backend-app" ]; then
    echo "❌ Erreur: Le répertoire 'backend-app' n'existe pas"
    echo "   Assurez-vous d'être dans le répertoire racine du projet"
    exit 1
fi

# Vérifier la connectivité SSH
echo "🔍 Vérification de la connectivité SSH..."
if ! ssh -o ConnectTimeout=10 -p $RASPBERRY_PORT $RASPBERRY_USER@$RASPBERRY_IP "echo 'Connexion SSH réussie'" 2>/dev/null; then
    echo "❌ Erreur: Impossible de se connecter au Raspberry Pi"
    echo "   Vérifiez l'IP, le nom d'utilisateur et que SSH est activé"
    exit 1
fi
echo "✅ Connexion SSH établie"

# Créer un répertoire temporaire
TEMP_DIR=$(mktemp -d)
echo "📁 Répertoire temporaire: $TEMP_DIR"

# Copier les fichiers du backend
echo "📦 Préparation des fichiers..."
cp -r backend-app/backend-api $TEMP_DIR/
cp -r backend-app/sidecar-python $TEMP_DIR/
cp backend-app/docker-compose.yml $TEMP_DIR/

# Créer un Dockerfile optimisé pour ARM64 (Raspberry Pi)
cat > $TEMP_DIR/backend-api/Dockerfile.arm64 << 'EOF'
# Dockerfile optimisé pour Raspberry Pi (ARM64)
FROM rust:1.75-slim as builder

# Installer les dépendances de build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier les fichiers de configuration
COPY Cargo.toml ./

# Créer un stub pour le build de cache
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build de cache des dépendances
RUN cargo build --release && rm -rf src

# Copier le code source
COPY src ./src

# Build final
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Installer les dépendances runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier le binaire
COPY --from=builder /app/target/release/crm-backend-api /app/crm-backend-api

# Créer un utilisateur non-root
RUN useradd -r -s /bin/false appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["/app/crm-backend-api"]
EOF

# Créer un docker-compose.yml pour Raspberry Pi
cat > $TEMP_DIR/docker-compose.pi.yml << 'EOF'
version: '3.8'

services:
  backend:
    build:
      context: ./backend-api
      dockerfile: Dockerfile.arm64
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://crm_user:crm_password@db:5432/crm_db
      - RUST_LOG=info
    depends_on:
      - db
    restart: unless-stopped

  sidecar:
    build:
      context: ./sidecar-python
      dockerfile: Dockerfile
    ports:
      - "8000:8000"
    environment:
      - PYTHONPATH=/app
    restart: unless-stopped

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=crm_db
      - POSTGRES_USER=crm_user
      - POSTGRES_PASSWORD=crm_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"
    restart: unless-stopped

volumes:
  postgres_data:
EOF

# Créer un Dockerfile pour le sidecar Python
cat > $TEMP_DIR/sidecar-python/Dockerfile << 'EOF'
FROM python:3.9-slim

WORKDIR /app

# Installer les dépendances système
RUN apt-get update && apt-get install -y \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Copier les requirements et installer les dépendances Python
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copier le code source
COPY . .

# Créer un utilisateur non-root
RUN useradd -r -s /bin/false appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8000

CMD ["python", "api.py"]
EOF

# Créer un script de déploiement Docker pour le Raspberry Pi
cat > $TEMP_DIR/deploy-docker-on-pi.sh << 'EOF'
#!/bin/bash

echo "🐳 Installation de Docker sur Raspberry Pi..."

# Installer Docker (si pas déjà installé)
if ! command -v docker &> /dev/null; then
    echo "📦 Installation de Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    sudo usermod -aG docker $USER
    rm get-docker.sh
fi

# Installer Docker Compose (si pas déjà installé)
if ! command -v docker-compose &> /dev/null; then
    echo "📦 Installation de Docker Compose..."
    sudo apt-get update
    sudo apt-get install -y docker-compose-plugin
fi

# Vérifier l'installation
echo "🔍 Vérification de l'installation Docker..."
docker --version
docker compose version

# Construire et démarrer les services
echo "🔨 Construction des images Docker..."
docker compose -f docker-compose.pi.yml build

echo "🚀 Démarrage des services..."
docker compose -f docker-compose.pi.yml up -d

# Afficher le statut
echo "📊 Statut des services:"
docker compose -f docker-compose.pi.yml ps

echo ""
echo "✅ Déploiement Docker terminé!"
echo "🌐 Backend accessible sur: http://localhost:8080"
echo "🐍 Sidecar accessible sur: http://localhost:8000"
echo "🗄️  Base de données PostgreSQL sur: localhost:5432"
echo ""
echo "📋 Commandes utiles:"
echo "  - Voir les logs: docker compose -f docker-compose.pi.yml logs -f"
echo "  - Arrêter: docker compose -f docker-compose.pi.yml down"
echo "  - Redémarrer: docker compose -f docker-compose.pi.yml restart"
EOF

chmod +x $TEMP_DIR/deploy-docker-on-pi.sh

# Transférer les fichiers sur le Raspberry Pi
echo "📤 Transfert des fichiers sur le Raspberry Pi..."
scp -P $RASPBERRY_PORT -r $TEMP_DIR/* $RASPBERRY_USER@$RASPBERRY_IP:~/crm-backend-docker/

# Exécuter le script de déploiement sur le Raspberry Pi
echo "🔧 Exécution du script de déploiement Docker sur le Raspberry Pi..."
ssh -p $RASPBERRY_PORT $RASPBERRY_USER@$RASPBERRY_IP "cd ~/crm-backend-docker && chmod +x deploy-docker-on-pi.sh && ./deploy-docker-on-pi.sh"

# Nettoyer le répertoire temporaire
rm -rf $TEMP_DIR

echo ""
echo "🎉 Déploiement Docker terminé!"
echo ""
echo "📋 Prochaines étapes:"
echo "1. Connectez-vous au Raspberry Pi: ssh $RASPBERRY_USER@$RASPBERRY_IP"
echo "2. Allez dans le répertoire: cd ~/crm-backend-docker"
echo "3. Vérifiez les logs: docker compose -f docker-compose.pi.yml logs -f"
echo ""
echo "🌐 Le backend sera accessible sur: http://$RASPBERRY_IP:8080"
echo "🐍 Le sidecar sera accessible sur: http://$RASPBERRY_IP:8000"
