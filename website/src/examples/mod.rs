use leptos::*;
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, RequestInit};

use crate::components::CodeBlock;

mod basic_dropdown;
mod basic_modal;

pub use basic_dropdown::BasicDropdown;
pub use basic_modal::BasicModal;

async fn get_code_example(name: &'static str) -> Result<String, JsValue> {
    let opts = RequestInit::new();
    let code_example_path = format!("/examples/{name}.rs");
    let request = Request::new_with_str_and_init(&code_example_path, &opts)?;
    let response = JsFuture::from(window().fetch_with_request(&request)).await?.dyn_into::<Response>()?;
    JsFuture::from(response.text()?).await?.as_string().ok_or(JsValue::UNDEFINED)
}

#[component]
pub fn RustCodeExample(name: &'static str) -> impl IntoView {
    let resource = create_local_resource(move || name, get_code_example);

    view! {
        <Suspense>
            {resource
                .get()
                .and_then(|result| result.ok())
                .map(|code_example| view! { <CodeBlock language="rust">{code_example}</CodeBlock> })}

        </Suspense>
    }
}
