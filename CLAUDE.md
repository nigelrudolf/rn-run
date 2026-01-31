# AI Usage Guide for rn-run

This document provides structured guidance for AI assistants (Claude, etc.) to effectively use `rn-run` for React Native development automation.

## Overview

`rn-run` is a CLI tool that automates React Native development workflows on macOS. It handles:
- Building and launching iOS/Android apps
- Cleaning build artifacts and dependencies
- Managing Metro bundler and simulators
- Environment diagnostics

**Always use `--json` flag** for programmatic parsing of results.

## Quick Reference

| Command | Purpose | Use When |
|---------|---------|----------|
| `rn-run --check-env --json` | Check dev environment | Build fails with "not found" errors |
| `rn-run --rn-version --json` | Get RN version | Need to determine npm vs yarn |
| `rn-run --list-simulators --json` | List iOS simulators | Need valid simulator name |
| `rn-run --list-emulators --json` | List Android emulators | Verify emulator exists |
| `rn-run --kill-metro` | Kill Metro bundler | Port 8081 in use, Metro stuck |
| `rn-run --quit-simulator` | Quit iOS Simulator | Simulator frozen/stuck |
| `rn-run --screenshot --json` | Screenshot iOS simulator | Capture current app state |
| `rn-run --screenshot -a --json` | Screenshot Android emulator | Capture current app state |
| `rn-run --update --json` | Update rn-run | Check for and install latest version |
| `rn-run --logs --json` | List build logs | View recent build logs for debugging |
| `rn-run --show-log --json` | Show latest log | Get full content of most recent build log |
| `rn-run --clean-modules` | Delete node_modules | Dependency corruption |
| `rn-run --clean-pods` | Clean iOS pods | Pod-related build errors |
| `rn-run --clean-gradle` | Clean Android gradle | Gradle sync failures |
| `rn-run --clean-metro` | Clear Metro cache | "Unable to resolve module" errors, stale bundle |
| `rn-run --pod-install` | Run pod install | After cleaning pods |
| `rn-run -i` | Run iOS app | Normal iOS development |
| `rn-run -a` | Run Android app | Normal Android development |
| `rn-run -i -c` | Clean install + run iOS | Dependency issues |
| `rn-run -i -u` | Deep clean + run iOS | RN version upgrades |

---

## Decision Trees

### iOS Build Fails

```
iOS build error detected
├── Error contains "pod" or "CocoaPods"?
│   ├── Yes → rn-run --clean-pods && rn-run --pod-install
│   └── Still failing? → rn-run -i -u (deep clean)
├── Error contains "Unable to find a specification"?
│   └── rn-run --clean-pods && rn-run --pod-install
├── Error contains "SDK not found" or "Xcode"?
│   └── rn-run --check-env --json (verify Xcode setup)
├── Error contains "node_modules" or dependency errors?
│   └── rn-run --clean-modules && npm install
├── Metro bundler issues (port 8081, stuck)?
│   └── rn-run --kill-metro
├── Error contains "Unable to resolve module" or stale bundle?
│   └── rn-run --clean-metro
└── Unknown error?
    └── rn-run -i -c (clean install) or rn-run -i -u (deep clean)
```

### Android Build Fails

```
Android build error detected
├── Error contains "gradle" or "Could not resolve"?
│   └── rn-run --clean-gradle
├── Error contains "SDK" or "ANDROID_HOME"?
│   └── rn-run --check-env --json (verify Android SDK)
├── Error contains "node_modules" or dependency errors?
│   └── rn-run --clean-modules && npm install
├── Metro bundler issues?
│   └── rn-run --kill-metro
├── Error contains "Unable to resolve module" or stale bundle?
│   └── rn-run --clean-metro
└── Unknown error?
    └── rn-run -a -c (clean install) or rn-run -a -u (deep clean)
```

### Environment Setup Issues

```
User reports build environment problems
├── First step → rn-run --check-env --json
├── Parse the JSON response:
│   ├── Check each item in "checks" array
│   ├── If check.ok == false:
│   │   └── Show check.error and check.fix to user
│   └── Focus on checks where required_for matches target platform
```

---

## JSON Response Formats

### Successful Response Structure

```json
{
  "command": "command-name",
  "success": true,
  "data": { /* command-specific payload */ }
}
```

### Error Response Structure

```json
{
  "command": "command-name",
  "success": false,
  "error": "Error description",
  "suggested_fix": "Recommended action to resolve"
}
```

### check-env Response

```json
{
  "command": "check-env",
  "success": true,
  "data": {
    "overall_status": "ok|warnings|errors",
    "checks": [
      {
        "name": "node",
        "ok": true,
        "version": "v20.10.0",
        "required_for": ["ios", "android"]
      },
      {
        "name": "cocoapods",
        "ok": false,
        "error": "CocoaPods not found",
        "fix": "Install CocoaPods: sudo gem install cocoapods OR brew install cocoapods",
        "required_for": ["ios"]
      }
    ],
    "summary": "Some required tools are missing..."
  }
}
```

### rn-version Response

```json
{
  "command": "rn-version",
  "success": true,
  "data": {
    "version": "0.74.1",
    "package_manager": "npm",
    "notes": ["RN 0.74+ uses npm and npx for commands"]
  }
}
```

### list-simulators Response

```json
{
  "command": "list-simulators",
  "success": true,
  "data": {
    "simulators": [
      {
        "name": "iPhone 15",
        "udid": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
        "state": "Shutdown",
        "runtime": "com.apple.CoreSimulator.SimRuntime.iOS-17-2"
      }
    ]
  }
}
```

### screenshot Response

```json
{
  "command": "screenshot",
  "success": true,
  "data": {
    "platform": "ios",
    "path": "screenshot-ios-1234567890.png",
    "message": "Screenshot saved to screenshot-ios-1234567890.png"
  }
}
```

### update Response

```json
{
  "command": "update",
  "success": true,
  "data": {
    "current_version": "0.1.7",
    "latest_version": "0.1.8",
    "updated": true,
    "message": "Updated from v0.1.7 to v0.1.8"
  }
}
```

### logs Response

```json
{
  "command": "logs",
  "success": true,
  "data": {
    "log_dir": "/Users/user/.rn-run/logs",
    "logs": [
      {
        "path": "/Users/user/.rn-run/logs/rn-run-ios-2024-01-30_14-30-00.log",
        "name": "rn-run-ios-2024-01-30_14-30-00.log",
        "size": 12345,
        "modified": "2024-01-30 14:30:00"
      }
    ]
  }
}
```

### show-log Response

```json
{
  "command": "show-log",
  "success": true,
  "data": {
    "path": "/Users/user/.rn-run/logs/rn-run-ios-2024-01-30_14-30-00.log",
    "content": "... full log content ..."
  }
}
```

---

## Common Error Patterns

| Error Pattern | Likely Cause | Fix Command |
|--------------|--------------|-------------|
| `Unable to find a specification for` | Stale pod cache | `--clean-pods && --pod-install` |
| `pod install` hangs or fails | Corrupted Pods | `--clean-pods && --pod-install` |
| `No bundle URL present` | Metro not running | `--kill-metro`, then rebuild |
| `Unable to resolve module` | Metro cache stale | `--clean-metro` |
| `ENOSPC: no space left` | Watchman limits | `--clean-modules` |
| `Could not resolve project :app` | Gradle cache | `--clean-gradle` |
| `SDK location not found` | ANDROID_HOME not set | Check `--check-env` |
| `error: Build input file cannot be found` | Stale build | `--clean-pods` |
| `The sandbox is not in sync with the Podfile.lock` | Pods out of sync | `--pod-install` |
| `node_modules/react-native` missing | Dependencies not installed | `npm install` or `yarn` |

---

## Recommended Workflow

### For AI Debugging React Native Issues

1. **Gather context first**
   ```bash
   rn-run --check-env --json    # Verify environment
   rn-run --rn-version --json   # Get RN version
   ```

2. **Parse error messages** from the user's build output

3. **Match against error patterns** (see table above)

4. **Execute targeted fix** before resorting to full clean:
   - Pod errors → `--clean-pods && --pod-install`
   - Gradle errors → `--clean-gradle`
   - Module errors → `--clean-modules && npm install`
   - Metro stuck → `--kill-metro`
   - Metro cache stale → `--clean-metro`

5. **Escalate if needed**:
   - If targeted fix fails → `rn-run -i -c` (clean install)
   - If clean install fails → `rn-run -i -u` (deep clean)

### For Fresh Project Setup

```bash
rn-run --check-env --json          # Verify all tools installed
# Fix any issues reported
rn-run -i -c                       # Clean install and run iOS
# OR
rn-run -a -c                       # Clean install and run Android
```

### For React Native Version Upgrades

```bash
rn-run -i -u                       # Deep clean + reinstall for iOS
# OR
rn-run -a -u                       # Deep clean + reinstall for Android
```

---

## Platform Notes

### iOS-specific
- Requires macOS with Xcode installed
- CocoaPods manages native dependencies
- Use `--simulator "iPhone 15 Pro"` to specify device
- `--list-simulators` shows available devices

### Android-specific
- Requires Android Studio with SDK
- ANDROID_HOME environment variable must be set
- Gradle manages native dependencies
- `--list-emulators` shows available devices

### Version-specific Behavior

| RN Version | Package Manager | Notes |
|------------|-----------------|-------|
| >= 0.74 | npm | Uses `npx react-native` |
| < 0.74 | yarn | Uses `yarn react-native` |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (check stderr or JSON error field) |

---

## Flags Reference

### Output
- `--json` - Output in JSON format (recommended for AI)

### Diagnostics
- `--check-env` - Check development environment
- `--rn-version` - Show React Native version
- `--list-simulators` - List iOS simulators
- `--list-emulators` - List Android emulators

### Process Management
- `--kill-metro` - Kill Metro bundler on port 8081
- `--quit-simulator` - Quit iOS Simulator
- `--screenshot` - Take screenshot of running simulator/emulator (iOS by default, use with `-a` for Android)
- `--output <path>` - Specify output path for screenshot (optional, defaults to timestamped filename)
- `--update` - Update rn-run to latest version from crates.io
- `--logs` - List recent build logs (last 10)
- `--show-log` - Show contents of most recent build log

### Cleanup
- `--clean-modules` - Delete node_modules only
- `--clean-pods` - Delete iOS Pods, Podfile.lock, build
- `--clean-gradle` - Delete Android build caches
- `--clean-metro` - Clear Metro bundler cache

### Build
- `--pod-install` - Run pod install

### Run
- `-i, --ios` - Run on iOS simulator
- `-a, --android` - Run on Android emulator
- `-s, --simulator <name>` - Specify iOS simulator
- `-c, --clean-install` - Clean and reinstall before running
- `-u, --upgrade` - Deep clean for RN upgrades
