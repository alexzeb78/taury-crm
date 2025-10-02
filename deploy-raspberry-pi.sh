#!/bin/bash

# Script de déploiement du backend sur Raspberry Pi
# Usage: ./deploy-raspberry-pi.sh [IP_RASPBERRY_PI] [USERNAME]

set -e

# Configuration par défaut
DEFAULT_IP="192.168.1.100"
DEFAULT_USER="pi"
DEFAULT_PORT="22"

# Paramètres
RASPBERRY_IP=${1:-$DEFAULT_IP}
RASPBERRY_USER=${2:-$DEFAULT_USER}
RASPBERRY_PORT=${3:-$DEFAULT_PORT}

echo "🚀 Déploiement du backend CRM sur Raspberry Pi"
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

# Créer un script de déploiement pour le Raspberry Pi
cat > $TEMP_DIR/deploy-on-pi.sh << 'EOF'
#!/bin/bash

echo "🔧 Installation des dépendances sur Raspberry Pi..."

# Mettre à jour le système
sudo apt-get update

# Installer Rust (si pas déjà installé)
if ! command -v cargo &> /dev/null; then
    echo "📦 Installation de Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
fi

# Installer les dépendances système
echo "📦 Installation des dépendances système..."
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    postgresql-client \
    python3 \
    python3-pip \
    python3-venv

# Installer Python dependencies
echo "🐍 Configuration de l'environnement Python..."
cd sidecar-python
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
deactivate
cd ..

# Compiler le backend Rust
echo "🦀 Compilation du backend Rust..."
cd backend-api
cargo build --release
cd ..

# Créer un script de démarrage
cat > start-backend.sh << 'STARTEOF'
#!/bin/bash

# Configuration
export DATABASE_URL="postgresql://username:password@localhost:5432/crm_db"
export RUST_LOG="info"
export PORT=8080

# Démarrer le backend Rust
echo "🚀 Démarrage du backend Rust..."
cd backend-api
./target/release/crm-backend-api &
BACKEND_PID=$!

# Démarrer le sidecar Python
echo "🐍 Démarrage du sidecar Python..."
cd ../sidecar-python
source venv/bin/activate
python api.py &
SIDECAR_PID=$!

# Fonction de nettoyage
cleanup() {
    echo "🛑 Arrêt des services..."
    kill $BACKEND_PID $SIDECAR_PID 2>/dev/null
    exit 0
}

# Capturer les signaux d'arrêt
trap cleanup SIGINT SIGTERM

# Attendre que les processus se terminent
wait $BACKEND_PID $SIDECAR_PID
STARTEOF

chmod +x start-backend.sh

echo "✅ Installation terminée!"
echo "🚀 Pour démarrer le backend, exécutez: ./start-backend.sh"
EOF

chmod +x $TEMP_DIR/deploy-on-pi.sh

# Transférer les fichiers sur le Raspberry Pi
echo "📤 Transfert des fichiers sur le Raspberry Pi..."
scp -P $RASPBERRY_PORT -r $TEMP_DIR/* $RASPBERRY_USER@$RASPBERRY_IP:~/crm-backend/

# Exécuter le script de déploiement sur le Raspberry Pi
echo "🔧 Exécution du script de déploiement sur le Raspberry Pi..."
ssh -p $RASPBERRY_PORT $RASPBERRY_USER@$RASPBERRY_IP "cd ~/crm-backend && chmod +x deploy-on-pi.sh && ./deploy-on-pi.sh"

# Nettoyer le répertoire temporaire
rm -rf $TEMP_DIR

echo ""
echo "🎉 Déploiement terminé!"
echo ""
echo "📋 Prochaines étapes:"
echo "1. Connectez-vous au Raspberry Pi: ssh $RASPBERRY_USER@$RASPBERRY_IP"
echo "2. Allez dans le répertoire: cd ~/crm-backend"
echo "3. Configurez la base de données PostgreSQL"
echo "4. Démarrez le backend: ./start-backend.sh"
echo ""
echo "🌐 Le backend sera accessible sur: http://$RASPBERRY_IP:8080"
