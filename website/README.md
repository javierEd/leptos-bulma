# Leptos Bulma website

URL: https://leptos-bulma.fermyon.app/

## Requirements

- Rust
- Node.js
- Spin v2.x

## Installation

Install Rust dependencies:

```sh
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install cargo-leptos
```

Install Node.js dependencies:

```sh
npm install -g sass
```

## Development deployment

Build and run

```sh
spin watch
```
