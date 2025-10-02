#!/bin/bash
# Script pour compiler l'API Python en exécutable standalone

echo "🔧 Building Python sidecar..."

# Installer PyInstaller si nécessaire
pip3 install pyinstaller

# Créer l'exécutable
pyinstaller --onefile \
    --name doc-generator \
    --add-data "template.docx:." \
    --hidden-import uvicorn.logging \
    --hidden-import uvicorn.loops \
    --hidden-import uvicorn.loops.auto \
    --hidden-import uvicorn.protocols \
    --hidden-import uvicorn.protocols.http \
    --hidden-import uvicorn.protocols.http.auto \
    --hidden-import uvicorn.protocols.websockets \
    --hidden-import uvicorn.protocols.websockets.auto \
    --hidden-import uvicorn.lifespan \
    --hidden-import uvicorn.lifespan.on \
    api.py

echo "✅ Sidecar compilé dans dist/doc-generator"
echo "📦 Copiez ce fichier dans src-tauri/binaries/"

