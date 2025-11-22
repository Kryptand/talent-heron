# GitHub Actions Cross-Platform Build Design

**Date:** 2025-11-22
**Status:** Approved

## Overview

Automated build system for the Archon Talent Updater Tauri app that builds for both Windows and macOS on every push to the main branch.

## Requirements

- **Trigger:** Automatic builds on every push to main branch
- **Platforms:** Windows and macOS
- **Artifacts:** Upload installers to workflow run (90-day retention)
- **Code Signing:** Not implemented (will be added later)

## Architecture

### Workflow Configuration

- **File Location:** `.github/workflows/build.yml`
- **Trigger:** Push to main branch
- **Runner Strategy:** Build matrix with platform-native runners
  - macOS builds on `macos-latest`
  - Windows builds on `windows-latest`
- **Parallelization:** Both platforms build simultaneously

### Build Matrix

```yaml
strategy:
  matrix:
    platform: [macos-latest, windows-latest]
```

This approach ensures builds run on their native platforms for maximum compatibility and reliability, avoiding cross-compilation issues.

### Build Steps

Each platform job executes:

1. **Checkout** - Clone repository code
2. **Setup Node.js** - Install Node.js v20 LTS with npm caching
3. **Setup Rust** - Install stable Rust toolchain
4. **Cache Dependencies** - Cache Cargo artifacts and npm modules
5. **Install Dependencies** - Run `npm install`
6. **Build Application** - Run `npm run tauri build`
7. **Upload Artifacts** - Upload installers to workflow run

### Dependency Caching

**Node.js caching:**
- Uses `actions/setup-node@v4` with `cache: 'npm'`
- Caches based on `package-lock.json`
- Saves ~1-2 minutes per build

**Rust caching:**
- Uses `Swatinem/rust-cache@v2`
- Caches Cargo dependencies and build artifacts
- Workspace: `src-tauri`
- Saves ~3-5 minutes per build after first run

**Expected build times:**
- First build: 5-10 minutes
- Cached builds: 2-3 minutes

### Platform-Specific Dependencies

**macOS:**
- No additional system dependencies required
- GitHub runner includes Xcode and necessary build tools

**Windows:**
- No additional system dependencies required
- GitHub runner includes Windows SDK

All Rust dependencies (reqwest, scraper, tokio, full_moon, etc.) compile during the build step.

## Build Outputs

### macOS Artifacts

Located in `src-tauri/target/release/bundle/`:
- `dmg/archon-config-updater-tauri_<version>_x64.dmg` - DMG installer
- `macos/archon-config-updater-tauri.app.tar.gz` - App bundle

### Windows Artifacts

Located in `src-tauri/target/release/bundle/`:
- `msi/archon-config-updater-tauri_<version>_x64_en-US.msi` - MSI installer
- `nsis/archon-config-updater-tauri_<version>_x64-setup.exe` - NSIS installer

### Artifact Upload

```yaml
- uses: actions/upload-artifact@v4
  with:
    name: ${{ matrix.platform }}-build
    path: src-tauri/target/release/bundle/
```

- Each platform gets separate artifact group
- Artifacts retained for 90 days
- Downloadable from GitHub Actions tab

## User Experience

### Running the Apps

**macOS:**
- Users will see Gatekeeper warning (unsigned app)
- Workaround: Right-click > Open, or System Settings > Privacy & Security

**Windows:**
- Users will see SmartScreen warning (unsigned app)
- Workaround: "More info" > "Run anyway"

### Accessing Builds

1. Navigate to repository Actions tab
2. Click on workflow run
3. Scroll to "Artifacts" section
4. Download platform-specific build

## Future Enhancements

### Code Signing (Not Implemented)

When ready to implement:

**macOS:**
- Apple Developer account required
- Store certificates in GitHub Secrets
- Add notarization step after build
- Use `@tauri-apps/action` with signing enabled

**Windows:**
- Code signing certificate required
- Store certificate and password in GitHub Secrets
- Sign during Tauri build process

### Additional Features

- Build on pull requests for validation
- Automated testing before build
- Version bumping automation
- Release creation on git tags

## Implementation Notes

- Keep workflow simple and maintainable
- Use official GitHub Actions where possible
- Avoid custom scripts unless necessary
- Follow Tauri's recommended build practices

## References

- [Tauri GitHub Actions Guide](https://tauri.app/v1/guides/building/github-actions)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)