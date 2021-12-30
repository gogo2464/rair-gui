extern crate druid;
use self::druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use self::druid::widget::Label;

pub fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}

pub fn build_main_gui() -> Result<(), PlatformError> {
    let start_up_windows = WindowDesc::new(build_ui());
    AppLauncher::with_window(start_up_windows).launch(())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_startup_ui() {
        let is_tart_up_windows_builded = build_main_gui();
        assert!(is_tart_up_windows_builded.is_ok());
    }
}
