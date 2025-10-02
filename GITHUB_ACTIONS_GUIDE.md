# Guide GitHub Actions - Taury CRM

## 🚀 Workflows Automatiques

### 1. **Build & Release** (`.github/workflows/build.yml`)
**Déclenchement :** Push sur main/master, Pull Request, Manuel

**Fonctionnalités :**
- ✅ Build frontend pour Windows, macOS, Linux
- ✅ Build backend Rust
- ✅ Build Python sidecar
- ✅ Création automatique de releases
- ✅ Upload des artefacts

**Résultat :**
- Frontend : `.msi`, `.dmg`, `.deb`
- Backend : Binaire Rust
- Sidecar : Archive Python

### 2. **Tests** (`.github/workflows/test.yml`)
**Déclenchement :** Push sur main/master, Pull Request

**Fonctionnalités :**
- ✅ Tests frontend (lint, type-check, build)
- ✅ Tests backend (unit tests, clippy, fmt)
- ✅ Tests Python sidecar
- ✅ Tests d'intégration
- ✅ Base de données PostgreSQL de test

### 3. **Sécurité** (`.github/workflows/security.yml`)
**Déclenchement :** Push, Pull Request, Hebdomadaire

**Fonctionnalités :**
- ✅ Scan de vulnérabilités (Trivy)
- ✅ Audit des dépendances (npm, cargo, pip)
- ✅ Analyse de code (CodeQL)
- ✅ Détection de secrets

### 4. **Déploiement** (`.github/workflows/deploy.yml`)
**Déclenchement :** Manuel uniquement

**Fonctionnalités :**
- ✅ Déploiement automatique sur Raspberry Pi
- ✅ Build pour ARM64
- ✅ Configuration systemd
- ✅ Vérification du déploiement

### 5. **Release** (`.github/workflows/release.yml`)
**Déclenchement :** Tag de version (v*)

**Fonctionnalités :**
- ✅ Génération automatique du changelog
- ✅ Build multi-plateforme
- ✅ Release complète avec documentation

## 📋 Utilisation

### Build Automatique
```bash
# Push sur main déclenche automatiquement le build
git push origin main
```

### Déploiement sur Raspberry Pi
1. Aller sur GitHub → Actions
2. Sélectionner "Deploy to Raspberry Pi"
3. Cliquer "Run workflow"
4. Renseigner l'IP du Pi et la clé SSH
5. Le déploiement se fait automatiquement

### Créer une Release
```bash
# Créer un tag
git tag v1.0.0
git push origin v1.0.0

# La release se crée automatiquement
```

## 🔧 Configuration Requise

### Secrets GitHub
Configurer dans Settings → Secrets and variables → Actions :

- `PI_SSH_KEY` : Clé SSH privée pour le Raspberry Pi
- `GITHUB_TOKEN` : Automatique

### Variables d'Environnement
- `DATABASE_URL` : URL de la base PostgreSQL
- `PORT` : Port du backend (défaut: 8080)

## 📊 Monitoring

### Status des Workflows
- Aller sur GitHub → Actions
- Voir l'historique des builds
- Télécharger les artefacts

### Logs
- Cliquer sur un workflow pour voir les logs
- Debug en cas d'erreur
- Télécharger les artefacts

## 🚨 Dépannage

### Build Frontend Échoue
- Vérifier les dépendances Node.js
- Vérifier la configuration Tauri
- Vérifier les erreurs de compilation

### Build Backend Échoue
- Vérifier les dépendances Rust
- Vérifier la configuration Cargo
- Vérifier les tests unitaires

### Déploiement Échoue
- Vérifier la connectivité SSH
- Vérifier les permissions
- Vérifier la configuration systemd

### Tests Échouent
- Vérifier la base de données de test
- Vérifier les dépendances
- Vérifier la configuration

## 📈 Améliorations Futures

### CI/CD Avancé
- [ ] Déploiement automatique sur staging
- [ ] Tests de performance
- [ ] Tests de charge
- [ ] Monitoring automatique

### Sécurité
- [ ] Scan de dépendances en temps réel
- [ ] Audit de code automatique
- [ ] Détection de vulnérabilités

### Déploiement
- [ ] Déploiement multi-environnements
- [ ] Rollback automatique
- [ ] Health checks
- [ ] Monitoring des performances

## 🎯 Résultat

Avec ces workflows, vous avez :
- ✅ **Build automatique** à chaque push
- ✅ **Tests automatiques** pour la qualité
- ✅ **Sécurité automatique** pour la protection
- ✅ **Déploiement automatique** sur Raspberry Pi
- ✅ **Releases automatiques** avec artefacts

**Tout est maintenant automatisé ! 🎉**
