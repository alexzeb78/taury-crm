# Python Sidecar - Document Generator

API Python locale intégrée dans l'application Tauri pour générer des documents Word.

## 🎯 Objectif

Permettre la génération de proposals Word **directement depuis l'exe/app** sans dépendances externes.

## 📋 Prérequis

1. Python 3.11+ installé
2. Votre fichier `template.docx` dans ce dossier

## 🔧 Setup (développement)

```bash
cd sidecar-python
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python api.py
```

L'API sera sur http://127.0.0.1:8001

## 📦 Build pour production

```bash
chmod +x build-sidecar.sh
./build-sidecar.sh
```

Cela créera `dist/doc-generator` (exécutable standalone)

## 🔗 Intégration dans Tauri

### Étape 1 : Copier l'exécutable

Après build, copiez :
- `dist/doc-generator` → `src-tauri/binaries/doc-generator-x86_64-apple-darwin` (macOS)
- `dist/doc-generator.exe` → `src-tauri/binaries/doc-generator-x86_64-pc-windows-msvc.exe` (Windows)

### Étape 2 : Configurer tauri.conf.json

Ajoutez dans `tauri` → `bundle` → `externalBin`:
```json
"externalBin": [
  "binaries/doc-generator"
]
```

### Étape 3 : Lancer depuis Rust

```rust
use tauri::api::process::{Command, CommandEvent};

let (mut rx, _child) = Command::new_sidecar("doc-generator")
    .expect("failed to create sidecar")
    .spawn()
    .expect("Failed to spawn sidecar");
```

## 🚀 Utilisation

Une fois configuré, Tauri lancera automatiquement l'API Python au démarrage.
L'application pourra alors appeler `http://localhost:8001/generate-word` pour créer des proposals !

## ⚠️ Important

- L'API écoute UNIQUEMENT sur 127.0.0.1 (localhost)
- Impossible d'y accéder depuis l'extérieur
- Sécurité maximale !

