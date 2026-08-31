# cdm-paint-rs

A feature-complete rewrite of [cdm_paint](https://github.com/aelsi2/cdm_paint/), a raster graphics editor based on CdM-16, using the Rust programming language.
Uses the [experimental CdM-16 Rust compiler](https://github.com/ylab-nsu/cdm16-rust) based on the [CdM-16 LLVM backend](https://github.com/ylab-nsu/cdm16-llvm-neo/).

### Hardware
- CdM-16 processor
- 32x32 screen, 1 bit per pixel
- D-pad + 4 buttons controller

### Software
- [x] Input
  - [x] Interrupt-based input
  - [x] Auto repeat
- [x] User interface:
  - [x] Main area
  - [x] Tool menu
- [x] Basic tools:
  - [x] Set pixel
  - [x] Clear screen
  - [x] Draw line
- [x] Shapes (outline + filled):
  - [x] Rectangle
  - [x] Ellipse
- [x] Flood fill tool
- [x] 16 operation drawing queue

## How to compile
Install the [Rust toolchain](https://github.com/ylab-nsu/cdm16-rust/releases) and set an override for the project. 
You can do the latter by running the following command once in the project directory (assuming `cdm` is the toolchain name you chose during installation):
```sh
rustup override set cdm
```

Run `cargo objcopy`:
```sh
cargo objcopy --release -- -O logisim logisim/cdm-paint.img
```

You will get a Logisim image located at `./logisim/cdm-paint.img`.

## How to run
Open the project at `./logisim/cdm-paint.circ` with [Logisim](https://sourceforge.net/projects/circuit/), set the tick frequency to the maximum possible value and run the simulation.
