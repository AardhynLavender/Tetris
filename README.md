# Tetris

Tetirs written in Rust and SDL2 for the Bachelor of Information Technology Game Development paper

## Installation

### Rust

You'll need to [Install Rust](https://www.rust-lang.org/tools/install)

### SDL2

you'll also need to install the SDL2 libraries. I've found using [VCPKG](https://github.com/microsoft/vcpkg) to be the simplist (and multiplatfor) method.

```bash
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```

> There are other installation methods not involving 3rd party software described in the [SDL2-Rust repository](https://github.com/Rust-SDL2/rust-sdl2)

## Compilation

```bash
cargo build
# or
cargo run
```
