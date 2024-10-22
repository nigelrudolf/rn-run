# rn-run

## Description: 

Run react native app on ios or android using a single command. This allows you to quickly run different branches without having to stop any processes or quit any programs, just run the command again on a different branch. Run in the root of your react native project.


## Usage: 
```
rn-run [OPTIONS]
```

## Options:
```
  -i, --ios                    Run iOS
  -a, --android                Run Android
  -s, --simulator <SIMULATOR>  
  -c, --clean-install          Clean install
  -h, --help                   Print help
  -V, --version                Print version
```

## Dependencies
This script needs the alias `ipad-mini` to work: https://github.com/nigelrudolf/useful-aliases

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
