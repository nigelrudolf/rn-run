# rn-run

## Description: 

Run react native app on ios or android using a single command. This allows you to quickly run different branches without having to stop any processes or quit any programs, just run the command again on a different branch. Run this script in the root of your react native project.

At this time, you do need to have Xcode opened to your workspace and you may also need to be running your debugger: https://github.com/jhen0409/react-native-debugger. Once these are opened and running you don't need to touch them again.


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

2. Clone repo

```
git clone https://github.com/nigelrudolf/rn-run.git ~/Downloads/rn-run
```

3. Build & Install
```
cargo install --path ~/Downloads/rn-run/
```

This will build and install the script to `~/.cargo/bin/` on MacOS and add it to your PATH. 
