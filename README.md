> [!WARNING]
> This branch uses the old `cocas`-based Rust toolchain that has been removed from the main branch of the CDM-16 Rust compiler.
> To compile this branch you'll have to use the compiler from the [`cdm-bitcode-linker`](https://github.com/ylab-nsu/cdm16-rust/tree/cdm-bitcode-linker) branch. 
> There are no prebuilt binaries for it, so you'll have to build it yourself.

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

`cd` into the project directory, create a virtual python environment at `./.venv` and install `cdm-devkit` in it:
```sh
cd <cdm_paint_repo>
python3 -m venv .venv
.venv/bin/pip install cdm-devkit
```

Execute:
```sh
cargo +cdm build
```

## How to run
Get the logisim project from the [C version](https://github.com/aelsi2/cdm_paint/), and edit the image path in the RAM component to the image built with `cargo` (located at `./target/cdm-none/debug/cdm_paint.img` relative to the project's root).
Run the simulation.
