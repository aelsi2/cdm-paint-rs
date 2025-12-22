# cdm-paint-rs

A rewrite of [cdm_paint](https://github.com/aelsi2/cdm_paint/), a CdM-16-based raster graphics editor using the Rust programming language.
Implements most of the featuers of the original (written in C). Uses the [experimental CdM-16 Rust compiler](https://github.com/ylab-nsu/cdm16-rust) based on the [CdM-16 LLVM backend](https://github.com/ylab-nsu/cdm16-llvm-neo/).

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
First, build the [LLVM libs](https://github.com/ylab-nsu/cdm16-llvm-neo/) and the [Rust compiler](https://github.com/ylab-nsu/cdm16-rust) and add the custom toolchain to rustup. Then build the `cdm-linker` crate, which is required by the compiler. 
It is not properply included in the rust toolchain at the moment, so you'll need to specify the path to the executable manually in `.cargo/config.toml`. 

Finally, `cd` into the project directory and execute:
```txt
cargo +<toolchain_name> build
```

## How to run
Get the logisim project from the [C version](https://github.com/aelsi2/cdm_paint/), and edit the image path in the RAM component to the image built with `cargo` (located at `./target/cdm-none/debug/cdm_paint.img` relative to the project's root).
Run the simulation.
