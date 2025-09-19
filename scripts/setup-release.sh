#!/bin/bash

set -euo pipefail

echo "🚀 Setting up ScoutQuest Semantic Release..."

# Check required tools
command -v node >/dev/null 2>&1 || { echo "❌ Node.js is required"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo is required"; exit 1; }
command -v pnpm >/dev/null 2>&1 || { echo "❌ pnpm is required"; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required"; exit 1; }

echo "✅ All required tools found"

# Install semantic-release dependencies
echo "📦 Installing semantic-release dependencies..."
pnpm install

# Setup git hooks for conventional commits (optional)
if command -v commitizen >/dev/null 2>&1; then
    echo "📝 Setting up commitizen for conventional commits..."
    pnpm add -g commitizen cz-conventional-changelog
    echo '{"path": "cz-conventional-changelog"}' > .czrc
fi

# Install JavaScript dependencies
echo "📦 Installing JavaScript SDK dependencies..."
cd scoutquest-js && pnpm install && cd ..
cd examples/js && pnpm install && cd ../..

echo "🧪 Running setup tests (unit tests only)..."
make test-setup

echo "🔨 Building all components..."
make build-all

echo "📝 Creating initial CHANGELOG..."
if [ ! -f CHANGELOG.md ]; then
    cat > CHANGELOG.md << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release with server, Rust SDK, and JavaScript SDK
- Comprehensive documentation website
- Docker support
- CI/CD pipeline with semantic releases

EOF
fi

echo "✅ ScoutQuest Semantic Release setup complete!"
echo ""
echo "🎯 Next steps:"
echo "  1. Set up GitHub secrets for releases:"
echo "     - GITHUB_TOKEN (automatic)"
echo "     - NPM_TOKEN (for JS SDK publishing)"
echo "     - CARGO_TOKEN (for Rust SDK publishing)"
echo "     - DOCKER_USERNAME & DOCKER_PASSWORD"
echo ""
echo "  2. Make your first commit with conventional format:"
echo "     git commit -m 'feat: initial ScoutQuest implementation'"
echo ""
echo "  3. Push to main branch to trigger first release:"
echo "     git push origin main"
echo ""
echo "📖 See CONTRIBUTING.md for commit conventions"
echo ""
echo "ℹ️  Note: Integration tests require the ScoutQuest server to be running."
echo "   Use 'make test-all' to run all tests including integration tests."
