# rn-run

## React Native Development Automation Tool for MacOS
rn-run is a command-line utility written in Rust that streamlines React Native development by automating the entire app launch process for both iOS and Android platforms.

## What It Does
rn-run is a "one-command solution" for React Native developers that eliminates the tedious manual steps typically required to run React Native apps. Instead of manually:

* Stopping existing Metro bundler processes
* Closing simulators
* Managing terminal windows
* Installing dependencies
* Starting the packager
* Launching the simulator/emulator

You simply run `rn-run --ios` or `rn-run --android` and the tool handles everything automatically.

## Usage: 
```
rn-run [OPTIONS]
```

## Options:
```
PLATFORM:
  -i, --ios                    Run iOS
  -a, --android                Run Android
  -s, --simulator <SIMULATOR>  Specify iOS simulator (default: iPhone 15)
  -c, --clean-install          Clean install before running
  -u, --upgrade                Aggressive cleanup for RN upgrades

DIAGNOSTICS:
  --check-env                  Check development environment setup
  --rn-version                 Show detected React Native version
  --list-simulators            List available iOS simulators
  --list-emulators             List available Android emulators

PROCESS MANAGEMENT:
  --kill-metro                 Kill Metro bundler on port 8081
  --quit-simulator             Quit iOS Simulator

CLEANUP:
  --clean-modules              Delete node_modules only
  --clean-pods                 Clean iOS Pods, Podfile.lock, build
  --clean-gradle               Clean Android build caches

BUILD:
  --pod-install                Run pod install

OUTPUT:
  --json                       Output in JSON format (for AI/automation)
  -h, --help                   Print help
  -V, --version                Print version
```

### Clean Install (`-c`)
Performs a standard clean install:
- Deletes `node_modules`
- Runs `npm install` or `yarn install`
- Runs `pod install` (iOS only)

### Upgrade Clean (`-u`)
Performs an aggressive cleanup for React Native upgrades:
- Everything in clean install, plus:
- **iOS:** Deletes `ios/Pods`, `ios/build`, `ios/Podfile.lock`, `package-lock.json`
- **Android:** Deletes `android/build`, `android/app/build`, `android/.gradle`, `package-lock.json`

## AI/Automation Usage

rn-run is designed to work with AI assistants like Claude. Use `--json` for structured output:

```bash
# Check environment (AI can parse and diagnose issues)
rn-run --check-env --json

# Get RN version (determines npm vs yarn)
rn-run --rn-version --json

# List available simulators
rn-run --list-simulators --json
```

See [CLAUDE.md](./CLAUDE.md) for comprehensive AI usage documentation, including:
- Decision trees for debugging build errors
- JSON response formats
- Common error patterns and fixes

## Setup

On MacOS you will need to set **Ask before closing** to **Never** in Terminal settings
![image](./media/terminal.png)

## Installation

1. Install Rust, more info: https://www.rust-lang.org/learn/get-started
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build & Install
```
cargo install rn-run
```

This will build and install the binary to `~/.cargo/bin/` on MacOS and add it to your PATH. 

## Support the Project

You can donate using Monero (XMR)

**Monero Address:** 
```
8AGHjrStt9EWEzKao7nvZNEGUHMHjWcJeWXts4wJsaog4eiE5Az4g2UjddiMLHLF6WPrKG2XT5rhcHrqqjTeedSo1RJZhNj
```
