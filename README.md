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

### Elements

- [x] Block
- [x] Box
- [x] Button
- [x] Icon
- [x] Notification
- [x] Table
- [x] Title

### Components

- [x] Dropdown
- [x] Menu
- [x] Modal
- [x] Navbar
- [x] Pagination

### Form

- [x] Input
- [x] Select
- [x] Textarea
- [x] File
- [x] Checkbox

### Columns

- [x] Columns
- [x] Column

### Layout

- [x] Section

## How to install

Add the following lines to your `Cargo.toml` file:

```toml
[build-dependencies]
# ···
leptos-bulma = { version = "0.0.0-alpha.13", default-features = false, features = ["build-script"] }

[dependencies]
# ···
leptos-bulma = "0.0.0-alpha.13"
```

Then add the following code to your `build.rs` file:

```rust
fn main() {
    // ···
    leptos_bulma::build("./style");
}
```

Use `leptos-bulma.scss` in your stylesheet:

```css
@use "leptos-bulma.scss";
```

And finally, add `leptos-bulma.scss` to your `.gitignore` file:

```
/style/leptos-bulma.scss
```
