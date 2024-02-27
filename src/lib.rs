use std::fs::File;
use std::io::Write;
use std::path::Path;

use leptos::ev::{Event, MouseEvent};
use leptos::LeptosOptions;

pub mod columns;
pub mod components;
pub mod elements;
pub mod form;

pub struct EventFn(Box<dyn Fn(Event) + 'static>);

impl<T> From<T> for EventFn
where
    T: Fn(Event) + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl EventFn {
    pub fn into_inner(self) -> Box<dyn Fn(Event) + 'static> {
        self.0
    }
}

pub struct MouseEventFn(Box<dyn Fn(MouseEvent) + 'static>);

impl<T> From<T> for MouseEventFn
where
    T: Fn(MouseEvent) + 'static,
{
    fn from(value: T) -> Self {
        Self(Box::new(value))
    }
}

impl MouseEventFn {
    pub fn into_inner(self) -> Box<dyn Fn(MouseEvent) + 'static> {
        self.0
    }
}

pub struct LeptosBulma;

impl LeptosBulma {
    pub fn setup(leptos_options: &LeptosOptions) {
        let target_path = Path::new(&leptos_options.site_root)
            .join(&leptos_options.site_pkg_dir)
            .join("leptos-bulma.css");
        let mut target_file = File::options()
            .create(true)
            .append(true)
            .open(target_path)
            .unwrap();

        let source_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("style/main.scss");
        let source_content = grass::from_path(source_path, &grass::Options::default()).unwrap();

        target_file.write_all(source_content.as_bytes()).unwrap();
    }
}
