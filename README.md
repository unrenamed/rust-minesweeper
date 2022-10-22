[![](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![](https://github.com/unrenamed/rust-minesweeper/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/unrenamed/rust-minesweeper/actions/workflows/build.yml)

# `Rust Minesweeper💣`

**Tech stack: Rust, WebAssembly, and Parcel**

This project uses pre-configured rust-parcel template with all the boilerplate
for compiling Rust to WebAssembly and hooking into a Parcel build pipeline.

- `npm run start` -- Serve the project locally for
  development at `http://localhost:1234`.

- `npm run build` -- Bundle the project (in production mode)

### Installation

To use the app locally you should install Rust and wasm-pack. The command below will do that for you:

```sh
./build.sh
```

Or if you want to do everything manually, follow the official Rust installation guide: https://www.rust-lang.org/tools/install

and then

```sh
cargo install wasm-pack
```
