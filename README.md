# Leptos Bulma

A [Leptos](https://leptos.dev) component library based on [Bulma](https://bulma.io) CSS framework.

[![crates.io](https://img.shields.io/crates/v/leptos-bulma.svg)](https://crates.io/crates/leptos-bulma)
[![docs.rs](https://docs.rs/leptos-bulma/badge.svg)](https://docs.rs/leptos-bulma)

[Website](https://leptos-bulma.fermyon.app) |
[Crates.io](https://crates.io/crates/leptos-bulma) |
[Docs.rs](https://docs.rs/leptos-bulma)

<p align="center">
    <img src="https://raw.githubusercontent.com/javierEd/leptos-bulma/main/bulma.jpg"/>
</p>

## THIS LIBRARY IS STILL A WORK IN PROGRESS!

And any suggestions are appreciated.

## What is mostly ready?

### Columns

- [x] Columns
- [x] Column

### Elements

- [x] Box
- [x] Button

### Components

- [x] Dropdown
- [x] Menu
- [x] Modal
- [x] Navbar

### Form

- [x] Input
- [x] Select
- [x] Textarea
- [x] File

## How to install

Run the following command to add the crate to your Leptos application:

```sh
cargo add leptos-bulma
```

Or add the following line to your `Cargo.toml` file:

```toml
leptos-bulma = "0.0.0-alpha.8"
```

Then add the following code to your `build.rs` file:

```rust
fn main() {
    // ···
    leptos_bulma::LeptosBulma::build("./style");
    // ···
}
```

Use `leptos-bulma.css` in your stylesheet:

```css
@use "leptos-bulma.css";
```

And finally, add `leptos-bulma.css` to your `.gitignore` file:

```
/style/leptos-bulma.css
```
