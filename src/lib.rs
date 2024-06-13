use leptos::ev::{Event, MouseEvent};

pub mod columns;
pub mod components;
pub mod elements;
pub mod enums;
pub mod form;
pub mod layout;

#[cfg(feature = "leptos-icons")]
pub mod icons {
    #[cfg(feature = "icondata-ai")]
    pub use icondata_ai;

    #[cfg(feature = "icondata-bi")]
    pub use icondata_bi;

    #[cfg(feature = "icondata-bs")]
    pub use icondata_bs;

    #[cfg(feature = "icondata-cg")]
    pub use icondata_cg;

    #[cfg(feature = "icondata-ch")]
    pub use icondata_ch;

    #[cfg(feature = "icondata-fa")]
    pub use icondata_fa;

    #[cfg(feature = "icondata-fi")]
    pub use icondata_fi;

    #[cfg(feature = "icondata-hi")]
    pub use icondata_hi;

    #[cfg(feature = "icondata-im")]
    pub use icondata_im;

    #[cfg(feature = "icondata-io")]
    pub use icondata_io;

    #[cfg(feature = "icondata-lu")]
    pub use icondata_lu;

    #[cfg(feature = "icondata-oc")]
    pub use icondata_oc;

    #[cfg(feature = "icondata-ri")]
    pub use icondata_ri;

    #[cfg(feature = "icondata-si")]
    pub use icondata_si;

    #[cfg(feature = "icondata-tb")]
    pub use icondata_tb;

    #[cfg(feature = "icondata-ti")]
    pub use icondata_ti;

    #[cfg(feature = "icondata-vs")]
    pub use icondata_vs;

    #[cfg(feature = "icondata-wi")]
    pub use icondata_wi;
}

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

#[cfg(feature = "build-script")]
pub fn build<P: AsRef<std::path::Path>>(output_dir: P) {
    use std::fs::{create_dir_all, read_to_string, File};
    use std::io::Write;
    use std::path::{Component, Path};
    use std::process::Command;

    let output_dir = output_dir.as_ref();
    let output_path = output_dir.join("leptos-bulma.scss");
    let main_scss_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("style/main.scss");

    let _ = create_dir_all(output_dir);

    let mut output_file = File::options()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_path)
        .expect("Could not create output file");

    Command::new("npm")
        .args(["--prefix", "./target", "install", "bulma@1.0"])
        .output()
        .expect("Could not install Bulma");

    let mut bulma_sass_prefix_dir = "".to_owned();

    for _ in output_dir.components().filter(|c| c != &Component::CurDir) {
        bulma_sass_prefix_dir += "../";
    }

    let use_bulma_sass = format!(
        "@forward \"{}target/node_modules/bulma/sass\";\n\n",
        bulma_sass_prefix_dir
    );
    let main_scss_content = read_to_string(main_scss_path).unwrap();
    let output_file_content = use_bulma_sass + main_scss_content.as_str();

    output_file
        .write_all(output_file_content.as_bytes())
        .expect("Could not write output file");
}
