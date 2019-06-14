# bk2-to-gbi-converter
Converts Game Boy inputs from BizHawk's bk2 format into timestamps for playback with Game Boy Interface

## Usage
```
bk2-to-gbi-converter rom-file.gbc inputs.bk2 > timestamps.txt
```

For more options, execute
```
bk2-to-gbi-converter --help
```

### Requirements
In order for this to work properly, the movie needs to use "CGB in GBA" mode, the movie may not use hard resets, and the game may not use joypad interrupts.


## Building
Building requires Rust Nightly
### Headless
```
cargo build --release
```
### With SDL output
This requires [SDL 2 development libraries](https://github.com/Rust-SDL2/rust-sdl2#requirements) to be installed.
```
cargo build --release --features "sdl"
```
