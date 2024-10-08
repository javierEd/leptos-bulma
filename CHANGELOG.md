# Changelog

## [0.6.0] - 2024-08-31

### Added

- Progress bar element (`BProgress`).
- More props to `BNotification`.
- Notification element page on the website.

## Changed

- Upgrade `leptos-use` to v0.13.
- Upgrade website dependencies.

## [0.5.0] - 2024-08-23

### Changed

- Upgrade `leptos-use` to v0.12.
- Upgrade website dependencies.

## [0.4.0] - 2024-06-13

### Added

- More icon packages from `icondata`.
- Move icon element to their own page on the website.

### Changed

- Use prop with enum `BSize` on `BIcon` and `BIconText`.

## [0.3.0] - 2024-06-06

### Added

- Tag element (`BTag` and `Btags`).
- Breadcrumb component (`BBreadcrumb` and `BBreadcrumbItem`).

### Changed

- Use class `is-active` in `BModal` to show it.
- Move button element to their own page on the website.

## [0.2.0] - 2024-05-13

### Added

- Enums for colors, states and sizes.
- More props to `BButton` and `BButtons`.
- More examples for buttons on the website.
- Anchor link button (`BAButton`).

### Changed

- Upgrade some dependencies on the website.

### Fixed

- Prop `class` on `BButton` and `BButtons` to avoid disabling other props on change.

## [0.1.0] - 2024-05-02

### Added

- Props for addons to `BTextField`.

### Fixed

- Leptos warning when getting field errors.

## [0.0.1] - 2024-04-29

### Fixed

- Apply some corrections and style improvements on the website.
- All props `class` should be `TextProp`.

## [0.0.0] - 2024-04-22

### Added

- Props `is_hoverable` and `href` to `BNavbarDropdownItem`.
- Prop `id` to `BTitle`.
- Installation guide for Leptos CSR on the website.
- Customization with Sass section on the website.

## [0.0.0-alpha.13] - 2024-04-19

### Changed

- Upgrade `leptos-use` in the website to include Spin feature for `use_cookie`.
- Replace theme context in the website with `leptos_use::use_color_mode`.
- Build script to generate a SCSS file that allows Bulma variables.

### Fixed

- Toggle footer images in the website with light and dark modes.

### Removed

- Custom `use_cookie` for SSR in the website.
- `indoc` dependency from the website.

## [0.0.0-alpha.12] - 2024-04-17

### Added

- Optional prop `is_checked` on checkbox.
- Icon Leptos components.
- More props to dropdown Leptos component.
- More props to button Leptos component.
- Buttons Leptos component.
- Button example on the website.
- Toggle between light and dark themes on the website.
- Show black overlay until website is loaded.

## [0.0.0-alpha.11] - 2024-04-13

### Changed

- Table Leptos components props.

## [0.0.0-alpha.10] - 2024-04-12

### Added

- Checkbox Leptos component.
- Block Leptos component.
- Title and subtitle Leptos components.
- Section Leptos component.
- Table Leptos components.

## [0.0.0-alpha.9] - 2024-04-10

### Added

- Optional callback when modal is closed.
- Pagination Leptos component.
- Columns page on the website;

### Fixed

- Some navbar props.
- Some columns and box props.

### Removed

- navbar markdown file.

## [0.0.0-alpha.8] - 2024-04-08

### Added

- A website running with Leptos to replace the mdBook.
- A guides page on the website.
- A components page on the website.
- Leptos Icons with Font Awesome package on the website.
- More props to modal Leptos component and included close button by default.
- Highlight.js for code examples.
- More props to form Leptos components.
- A form page on the website.
- An elements page on the website.

### Changed

- Small changes on the build method.

### Removed

- Leptos Router dependency.
- mdBook folder.

## [0.0.0-alpha.7] - 2024-03-27

### Added

- More props to navbar Leptos components.
- More props to form Leptos components.
- Leptos Icons with Font Awesome package.

### Changed

- Upgrade to Bulma v1.0.

## [0.0.0-alpha.6] - 2024-03-13

### Changed

- Compress generated CSS file content.

### Fixed

- Use File.write instead of append to generate CSS file.

## [0.0.0-alpha.5] - 2024-02-27

### Added

- Button Leptos component.
- Notification Leptos component.
- Columns Leptos components.
- Menu Leptos components.
- A method to setup the library and generate the leptos-bulma.css file.
- File Leptos component.

### Changed

- Upgrade dependencies.
- Use Show Leptos component to toggle modal.

### Fixed

- Form field values.

## [0.0.0-alpha.4] - 2024-01-29

### Added

- Some Leptos components for forms (Input, Select and Textarea).

## [0.0.0-alpha.3] - 2024-01-26

### Added

- Custom style.css for the book.
- Navbar dropdown Leptos components.
- Dropdown Leptos components.
- Introduction page to the book.

### Fixed

- Exclude .editorconfig and leptosfmt.toml files from `cargo publish`.
- Exclude Cargo.lock file from git repository and `cargo publish`.
- Modal code example in the book.

### Removed

- Styles from the book iframes.
- Cargo.lock file.

## [0.0.0-alpha.2] - 2024-01-25

### Added

- Build and deploy the book with mdBook and GitHub Actions.
- More Navbar Leptos components.
- Modal Leptos components.
- Class argument to Box Leptos component.
- leptosfmt.toml file.

### Changed

- Allow older versions of Leptos 0.5.x

## [0.0.0-alpha.1] - 2024-01-25

### Added

- Everything is new.
