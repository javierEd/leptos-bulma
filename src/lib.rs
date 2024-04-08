use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

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
        let output_dir = Path::new(&leptos_options.site_root).join(&leptos_options.site_pkg_dir);
        Self::build(output_dir)
    }

    pub fn build<P: AsRef<Path>>(output_dir: P) {
        let output_path = output_dir.as_ref().join("leptos-bulma.css");

        let _ = create_dir_all(output_dir);

        let mut output_file = File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(output_path)
            .expect("Could not create output file");

        let _ = Command::new("npm")
            .args(["--prefix", "./target", "install", "bulma@1.0"])
            .output();

        let source_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("style/main.scss");
        let source_content = grass::from_path(
            source_path,
            &grass::Options::default()
                .style(grass::OutputStyle::Compressed)
                .load_path("./target/node_modules/bulma")
                .allows_charset(true),
        )
        .unwrap();

        output_file.write_all(source_content.as_bytes()).unwrap();
    }
}
