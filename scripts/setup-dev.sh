#!/bin/bash

echo "🚀 Setting up ScoutQuest development environment..."

# Fonction pour installer pre-commit
install_precommit() {
    echo "📦 Installing pre-commit..."

    if [[ "$OSTYPE" == "darwin"* ]] && command -v brew &> /dev/null; then
        brew install pre-commit
    elif command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v pip &> /dev/null; then
        pip install pre-commit
    else
        echo "❌ Cannot install pre-commit. Please install Python/pip first."
        exit 1
    fi
}

# Vérifier si pre-commit est installé
if ! command -v pre-commit &> /dev/null; then
    install_precommit
fi

# Installer les git hooks
echo "🪝 Installing git hooks..."
pre-commit install

# Setup semantic release si pas déjà fait
if [ ! -f "CHANGELOG.md" ]; then
    echo "🔧 Running semantic release setup..."
    ./scripts/setup-release.sh
else
    echo "✅ Semantic release already configured"
fi

echo ""
echo "✅ Development environment ready!"
echo ""
echo "📋 Next steps:"
echo "  • Pre-commit hooks are now active"
echo "  • Code will be checked before each commit"
echo "  • Run 'make test-setup' to verify everything works"
echo ""
echo "🛠️  Useful commands:"
echo "  • make test-setup    - Run tests (unit only)"
echo "  • make test-all      - Run all tests (requires server)"
echo "  • make build-all     - Build all components"
echo "  • pre-commit run -a  - Run hooks on all files"
