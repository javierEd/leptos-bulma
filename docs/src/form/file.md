# File

Example:

```rust
use leptos::*;
use leptos_bulma::form::BFile;

#[compoment]
pub fn MyFile() -> impl IntoView {
    view! {
        <BFile accept="image/jpeg,image/png" id="file" name="file" label="Select a file" multiple=true/>
    }
}
```
