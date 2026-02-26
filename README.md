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
Firstly, build the [LLVM libs](https://github.com/ylab-nsu/cdm16-llvm-neo/) and the [Rust compiler](https://github.com/ylab-nsu/cdm16-rust) and add the custom toolchain to rustup like this:
```sh
rustup toolchain link cdm <rust_repo>/build/host/stage1
```

Rust uses the C compiler from the CDM LLVM distribution as a linker and it needs to find it. Do either of these:
- specify the linker path in `.cargo/config.toml`:
```toml
[target.cdm-none]
linker = "<replace this with the path to clang>"
```
- make the compiler available under the name `clang` by temporarily adding the LLVM binary directory to `$PATH`.

Execute:
```sh
cargo +cdm build
```

You will get a binary located at `./target/cdm-none/debug/cdm_paint`. Convert it to a Logisim image with this command:
```sh
{ echo 'v2.0 raw'; od -tx1 -An -v | tr -s '[:blank:]' '\n'; } < ./target/cdm-none/debug/cdm_paint > ./target/cdm-none/debug/cdm_paint.img
```

## How to run
Get the logisim project from the [C version](https://github.com/aelsi2/cdm_paint/). Edit the image path in the RAM component and change it to the Logisim image you got.
Run the simulation.
