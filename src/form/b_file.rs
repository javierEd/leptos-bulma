use leptos::html::Input;
use leptos::*;
use web_sys::{Event, MouseEvent};

use crate::EventFn;

#[component]
pub fn BFile(
    #[prop(optional)] accept: Option<&'static str>,
    #[prop(optional, into)] icon: Option<ViewFn>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(default = false)] multiple: bool,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] on_change: Option<EventFn>,
) -> impl IntoView {
    let node_ref = create_node_ref::<Input>();
    let has_file = create_rw_signal(false);
    let file_name = create_rw_signal("".to_owned());

    let input_on_change = move |event| {
        let element_files = node_ref.get().and_then(|element| element.files());

        has_file.set(
            element_files
                .as_ref()
                .map(|f| f.length() > 0)
                .unwrap_or(false),
        );

        if let Some(files) = element_files {
            let mut names = "".to_owned();

            for index in 0..files.length() {
                if let Some(name) = files.get(index).map(|file| file.name()) {
                    names.push_str(&name);

                    if index < files.length() - 1 {
                        names.push_str(", ");
                    }
                }
            }

            file_name.set(names);
        } else {
            file_name.set("".to_owned());
        }

        if let Some(oc) = on_change.as_ref() {
            oc.0(event)
        }
    };

    let clear_files = move |event: MouseEvent| {
        event.prevent_default();

        if let Some(element) = node_ref.get() {
            element.set_value("");
            let event = Event::new("change").unwrap();
            event.init_event_with_bubbles("change", true);
            let _ = element.dispatch_event(&event);
        }
    };

    view! {
        <div class="file has-name is-fullwidth" class:has-file=has_file>
            <label class="file-label">
                <input
                    node_ref=node_ref
                    accept=accept
                    class="file-input"
                    id=id
                    multiple=multiple
                    name=name
                    type="file"
                    on:change=input_on_change
                />

                <span class="file-cta">
                    {icon.map(|icon| view! { <span class="file-icon">{icon.run()}</span> })}
                    <span class="file-label">{label}</span>
                </span>

                <span class="file-name">{file_name}</span>

                <Show when=move || has_file.get()>
                    <a class="button file-clear" title="Clear" on:click=clear_files>
                        <span class="icon">"☓"</span>
                    </a>
                </Show>
            </label>
        </div>
    }
}
