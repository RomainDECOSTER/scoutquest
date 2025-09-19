# Conventional Commits Configuration

## Commit Types

- `feat`: New features (triggers minor version bump)
- `fix`: Bug fixes (triggers patch version bump)
- `perf`: Performance improvements (triggers patch version bump)
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring without functional changes
- `test`: Adding or updating tests
- `chore`: Build process or auxiliary tool changes
- `ci`: CI/CD changes

## Breaking Changes

Add `BREAKING CHANGE:` in the commit body or `!` after type to trigger major version bump:

```
feat!: remove loadbalancer support
feat(api)!: change endpoint structure

BREAKING CHANGE: LoadBalancer class removed from all SDKs
```

## Scopes

Use these scopes to identify which component is affected:

- `server`: ScoutQuest server changes
- `js-sdk`: JavaScript/TypeScript SDK changes
- `rust-sdk`: Rust SDK changes
- `docs`: Documentation website changes
- `examples`: Example applications changes
- `ci`: CI/CD and build system changes

## Examples

```bash
# New feature in server (minor bump)
git commit -m "feat(server): add health check endpoint"

# Bug fix in JS SDK (patch bump)
git commit -m "fix(js-sdk): handle connection timeout correctly"

# Breaking change in Rust SDK (major bump)
git commit -m "feat(rust-sdk)!: simplify client API

BREAKING CHANGE: ServiceDiscoveryClient constructor now takes single URL parameter"

# Documentation update (no version bump)
git commit -m "docs: add installation guide"

# Chore work (no version bump)
git commit -m "chore: update dependencies"
```

## Release Process

1. **Automatic**: Push to main/develop triggers semantic-release
2. **Manual**: Run `make release-prepare VERSION=x.y.z` then push
3. **Dry run**: Use `npm run release:dry-run` to preview changes

## Version Synchronization

All components (server, JS SDK, Rust SDK, docs) use the same version number for consistency and easier tracking.
