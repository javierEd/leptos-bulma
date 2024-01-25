# Modal

Example:

```rust
use leptos::*;
use leptos_bulma::components::{BModal, BModalClose, BModalContent};
use leptos_bulma::elements::BBox;

#[component]
pub fn MyModal() -> impl IntoView {
    let (is_active, set_is_active) = create_signal(true);

    view !{
        <BModal is_active=is_active>
            <BModalContent>
                <BBox class="has-text-centered">
                    "Hello, World!"
                </BBox>
            </BModalContent>
            <BModalClose  on_click = move |_| set_is_active(false)></BModalClose>
        </BModal>
    }
}
```

See it in action:

<iframe src="https://codesandbox.io/p/devbox/leptos-bulma-modal-4crtz7?file=%2Fsrc%2Fmain.rs&embed=1"
     title="Leptos Bulma Modal"
     sandbox="allow-forms allow-modals allow-popups allow-presentation allow-same-origin allow-scripts"
   ></iframe>
