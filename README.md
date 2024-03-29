# Leptos Bulma

A [Leptos](https://leptos.dev) component library based on [Bulma](https://bulma.io) CSS framework.

[![crates.io](https://img.shields.io/crates/v/leptos-bulma.svg)](https://crates.io/crates/leptos-bulma)
[![docs.rs](https://docs.rs/leptos-bulma/badge.svg)](https://docs.rs/leptos-bulma)

[Book](https://javiered.github.io/leptos-bulma) |
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

Add the crate to your Leptos project:

```sh
cargo add leptos-bulma
```

Then add the following code to your main function:

```rust
async fn main() -> std::io::Result<()> {
    ···
    leptos_bulma::LeptosBulma::setup(&conf.leptos_options);
    ···
}
```

Finally add this to your stylesheet:

```css
@import "leptos-bulma.css";
```
