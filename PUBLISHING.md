# ScoutQuest Publishing Guide

Ce document explique le nouveau processus de publication pour ScoutQuest.

## 🚀 Processus Automatique (Recommandé)

### 1. Release avec semantic-release
Le workflow `release.yml` s'exécute automatiquement sur la branche `main` :
- Analyse les commits pour déterminer la version
- Met à jour les versions dans tous les packages
- Génère le changelog
- Crée un tag Git
- ⚠️ **Ne publie plus automatiquement** les packages

### 2. Publication automatique sur tag
Le workflow `publish.yml` se déclenche automatiquement quand un tag `v*` est créé :
- Publie sur NPM (scoutquest-js)
- Publie sur Crates.io (scoutquest-rust)
- Publie sur Docker Hub (scoutquest-server)
- Déploie la documentation

## 🔧 Processus Manuel

### Via GitHub Actions (Interface Web)
1. Aller dans l'onglet "Actions" du repo
2. Sélectionner le workflow "Publish"
3. Cliquer sur "Run workflow"
4. Saisir la version et choisir les composants à publier

### Via ligne de commande

#### Script tout-en-un
```bash
# Publier tous les composants
./scripts/publish.sh all 1.2.3

# Publier un composant spécifique
./scripts/publish.sh npm 1.2.3
./scripts/publish.sh cargo 1.2.3
./scripts/publish.sh docker 1.2.3
./scripts/publish.sh docs 1.2.3
```

#### Commandes Makefile
```bash
# Publier tous les composants
make release-publish VERSION=1.2.3

# Publier individuellement
make publish-npm VERSION=1.2.3
make publish-cargo VERSION=1.2.3
make publish-docker VERSION=1.2.3
make publish-docs VERSION=1.2.3
```

#### Scripts NPM
```bash
# Publier tous les composants
pnpm publish:all

# Publier individuellement
pnpm publish:npm
pnpm publish:cargo
pnpm publish:docker
pnpm publish:docs
```

## 🔑 Variables d'environnement requises

### Pour GitHub Actions
- `NPM_TOKEN` : Token NPM pour publier les packages
- `CARGO_TOKEN` : Token Cargo pour publier les crates
- `DOCKER_USERNAME` / `DOCKER_PASSWORD` : Credentials Docker Hub
- `GITHUB_TOKEN` : Fourni automatiquement par GitHub

### Pour la publication locale
- NPM : `npm login` ou `pnpm login`
- Cargo : `cargo login`
- Docker : `docker login`

## 🎯 Avantages du nouveau système

1. **Arbre Git propre** : Les publications se font après la création du tag
2. **Flexibilité** : Possibilité de publier manuellement des composants individuels
3. **Retry facile** : En cas d'échec, on peut relancer la publication sans refaire la release
4. **Parallélisation** : Les publications se font en parallèle
5. **Logs séparés** : Chaque composant a ses propres logs d'erreur

## 🚨 Résolution des problèmes courants

### Publication NPM échoue
```bash
# Vérifier l'authentification
pnpm login

# Publier manuellement avec debug
cd scoutquest-js
pnpm publish --access public --no-git-checks --dry-run
```

### Publication Cargo échoue
```bash
# Vérifier l'authentification
cargo login

# Publier manuellement avec debug
cd scoutquest-rust
cargo publish --dry-run
```

### Publication Docker échoue
```bash
# Vérifier l'authentification
docker login

# Build et push manuels
docker build -t scoutquest/server:latest ./scoutquest-server
docker push scoutquest/server:latest
```
