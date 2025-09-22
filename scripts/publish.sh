#!/bin/bash

# ScoutQuest Manual Publishing Script
# Usage: ./scripts/publish.sh [npm|cargo|docker|docs|all] [version]

set -e

COMPONENT=${1:-all}
VERSION=${2:-$(cat package.json | grep '"version"' | head -1 | awk -F: '{ print $2 }' | sed 's/[",]//g' | tr -d '[[:space:]]')}

echo "🚀 ScoutQuest Manual Publishing Script"
echo "Component: $COMPONENT"
echo "Version: $VERSION"
echo ""

# Validation
if [[ ! "$COMPONENT" =~ ^(npm|cargo|docker|docs|all)$ ]]; then
    echo "❌ Invalid component. Use: npm, cargo, docker, docs, or all"
    exit 1
fi

# Confirmation
read -p "🤔 Are you sure you want to publish $COMPONENT v$VERSION? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Publishing cancelled"
    exit 1
fi

# Check prerequisites
echo "🔍 Checking prerequisites..."

if [[ "$COMPONENT" == "npm" || "$COMPONENT" == "all" ]]; then
    if ! command -v pnpm >/dev/null 2>&1; then
        echo "❌ pnpm not found. Please install pnpm first."
        exit 1
    fi

    if [[ -z "${NPM_TOKEN:-}" ]]; then
        echo "⚠️  NPM_TOKEN not set. Make sure you're logged in with 'npm login' or 'pnpm login'"
    fi
fi

if [[ "$COMPONENT" == "cargo" || "$COMPONENT" == "all" ]]; then
    if ! command -v cargo >/dev/null 2>&1; then
        echo "❌ Cargo not found. Please install Rust first."
        exit 1
    fi

    if [[ -z "${CARGO_TOKEN:-}" ]] && [[ ! -f ~/.cargo/credentials ]]; then
        echo "⚠️  Cargo credentials not found. Run 'cargo login' first."
    fi
fi

if [[ "$COMPONENT" == "docker" || "$COMPONENT" == "all" ]]; then
    if ! command -v docker >/dev/null 2>&1; then
        echo "❌ Docker not found. Please install Docker first."
        exit 1
    fi

    if ! docker info >/dev/null 2>&1; then
        echo "❌ Docker daemon not running. Please start Docker first."
        exit 1
    fi
fi

echo "✅ Prerequisites check passed"
echo ""

# Publishing
case $COMPONENT in
    npm)
        echo "📦 Publishing NPM package..."
        make publish-npm VERSION=$VERSION
        ;;
    cargo)
        echo "📦 Publishing Cargo crate..."
        make publish-cargo VERSION=$VERSION
        ;;
    docker)
        echo "🐳 Publishing Docker image..."
        make publish-docker VERSION=$VERSION
        ;;
    docs)
        echo "📚 Publishing documentation..."
        make publish-docs VERSION=$VERSION
        ;;
    all)
        echo "📦 Publishing all components..."
        make release-publish VERSION=$VERSION
        ;;
esac

echo ""
echo "✅ Publishing completed successfully!"
echo "🎉 $COMPONENT v$VERSION is now available!"

# Optional: Open relevant pages
case $COMPONENT in
    npm)
        echo "📎 NPM: https://www.npmjs.com/package/scoutquest"
        ;;
    cargo)
        echo "📎 Crates.io: https://crates.io/crates/scoutquest"
        ;;
    docker)
        echo "📎 Docker Hub: https://hub.docker.com/r/scoutquest/server"
        ;;
    docs)
        echo "📎 Documentation: https://romaindecoster.github.io/scoutquest/"
        ;;
    all)
        echo "📎 NPM: https://www.npmjs.com/package/scoutquest"
        echo "📎 Crates.io: https://crates.io/crates/scoutquest"
        echo "📎 Docker Hub: https://hub.docker.com/r/scoutquest/server"
        echo "📎 Documentation: https://romaindecoster.github.io/scoutquest/"
        ;;
esac
