# Tetris

Tetris clone for a Bachelor of Information Technology Game Development paper assignment.

## Installation

### Rust

You will need to [Install Rust](https://www.rust-lang.org/tools/install)

### SDL2

You will need to install the SDL2 libraries.

I've found using [Microsoft VCPKG](https://github.com/microsoft/vcpkg) to be a simple (and multiplatform) method.

```bash
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```

> There are other installation methods not involving VCPKG described in
> the [SDL2-Rust Repository](https://github.com/Rust-SDL2/rust-sdl2) but I've not tested any of these.
> Remember to remove the VCPKG references in `Cargo.toml` file if you do so.

#### Enabled SDL2 `unsafe_textures`

I've enabled the `unsafe_textures` feature for the `sdl2` crate in `Cargo.toml` which removes the generic lifetime
annotations for the `sdl2::rendering::Texture` struct.

This removes the need to propagate lifetimes throughout the codebase when dealing with textures. As textures are dropped
before their "*owning*" `TextureCreator`, there is no risk of dangling references or memory leaks.

## Download

Download the latest precompiled executable for your platform
under [Releases](https://github.com/AardhynLavender/Tetris/releases)

## Compilation

```bash
cargo build
# or
cargo run
```
