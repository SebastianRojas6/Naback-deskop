slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = Menu::new()?;

    main_window.run()
}