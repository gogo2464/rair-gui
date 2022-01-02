extern crate druid;
use self::druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, Env, Lens, LocalizedString, WidgetExt};
use self::druid::widget::{Align, Flex, Label, TextBox};

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Open the file");

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    name: String,
}

/// # build_windows_to_open_binary
///
/// Builds a label with the text passed as argument.

pub fn build_windows_to_open_binary() -> WindowDesc<HelloState> {
    let label = Label::new("Hello world");
    let main_window = WindowDesc::new(label)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));
    main_window
}

/// # build_main_gui
///
/// Builds everything.

pub fn build_main_gui() -> Result<(), PlatformError> {
    
    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };
    
    
    let start_up_windows = build_windows_to_open_binary();
    AppLauncher::with_window(start_up_windows).launch(initial_state)?;
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
