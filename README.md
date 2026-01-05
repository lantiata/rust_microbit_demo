# rust_microbit_demo
Blinking LED rust demo for microbit v2

# HW
https://makecode.microbit.org/device/v2
nRF52833 

#RUST
https://rust-lang.org/tools/install/
## Cortex M4 support
rustup target add thumbv7em-none-eabihf

## LLVM tools
rustup component add llvm-tools

## Binutils
cargo install cargo-binutils

## Debug tool
cargo install probe-rs-tools --locked

## RTT support
cargo add rtt_target

cargo add cortex-m --features critical-section-single-core

#Links
##Rust Platform Support 
https://doc.rust-lang.org/beta/rustc/...

##Crates Registry 
https://crates.io/
##Arm GNU Toolchain 
https://developer.arm.com/downloads/
##The Embedded Rust book 
https://docs.rust-embedded.org/book/
##"The Book" 
https://doc.rust-lang.org/book/index.

#IDE 
Version: 1.107.1 (user setup)
Commit: 994fd12f8d3a5aa16f17d42c041e5809167e845a
Date: 2025-12-17T14:15:14.850Z
Electron: 39.2.3
ElectronBuildId: 12895514
Chromium: 142.0.7444.175
Node.js: 22.21.1
V8: 14.2.231.21-electron.0
OS: Windows_NT x64 10.0.19045

## Extensions
rust-analyzer
Dependi
ERBD
Error Lens
Even Better TOML

# Debugging
##Start command from a new terminal:
arm-none-eabi-gdb.exe .\target\thumbv7m-none-eabi\debug\rustymicrobit

## Connect
target remote :1337

##Misc gdb commands
brake main.rs:38 //brakepoint to main line 38.
continue         // Continue running
print count      // get variable 
info break       // List brakepoints
delete 1         // delete 1st brakepoint
monitor reset    // Reset controller