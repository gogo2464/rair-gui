extern crate druid;
use self::druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, 
                  Lens, LocalizedString, UnitPoint, ImageBuf
                  };
                  
use self::druid::widget::{Align, Image, Flex, Button, FillStrat};

/// # WINDOW_TITLE
///
/// Title of the application.

const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Open the file");

/// # HelloState
///
/// State of the application.

#[derive(Clone, Data, Lens)]
pub struct HelloState {
    name: String,
}

/// # build_flex_logo_and_about
///
/// Builds the flex with the rair log and the about button.

fn build_flex_logo_and_about() -> Flex <HelloState> {
    Flex::column()
        .with_child(build_logo())
        .with_child(Button::new("about"))
}

/// # build_logo
///
/// Builds the rair logo (currently the odcoder png)

fn build_logo() -> impl Widget<HelloState> {
    let logo_oddcoder = ImageBuf::from_data(include_bytes!("../data/oddcoder.png")).unwrap();
    let img = Image::new(logo_oddcoder).fill_mode(FillStrat::Fill);
    let centered = Align::horizontal(UnitPoint::TOP, img);
    centered
}

/// # build_windows_to_open_binary
///
/// Builds a label with the text passed as argument.

fn build_windows_to_open_binary() -> WindowDesc<HelloState> {
    let main_window = WindowDesc::new(build_flex_logo_and_about)
        .title(WINDOW_TITLE)
        .window_size((600.0, 700.0));
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
