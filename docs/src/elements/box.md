# Box

Example:

```rust
use leptos::*;
use leptos_bulma::elements::BBox;

#[compoment]
pub fn MyBox() -> impl IntoView {
    view! {
        <BBox class="has-text-centered m-4">
            "Hello, World!"
        </BBox>
    }
}
```

See it in action:

<iframe src="https://codesandbox.io/p/devbox/leptos-bulma-box-tk2yk4?embed=1&file=%2Fsrc%2Fmain.rs"
     style="width:100%; height: 500px; border:0; border-radius: 4px; overflow:hidden;"
     title="Leptos Bulma Box"
     sandbox="allow-forms allow-modals allow-popups allow-presentation allow-same-origin allow-scripts"
   ></iframe>
