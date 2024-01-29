# Select

Example:

```rust
use leptos::*;
use leptos_bulma::form::BSelect;

#[compoment]
pub fn MySelect() -> impl IntoView {
    view! {
        <BSelect options=vec![("Option 1", "1"), ("Option 2", "2")] />
    }
}
```
