extern crate druid;
extern crate druid_widget_nursery;
use self::druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, 
                  Lens, LocalizedString, UnitPoint, ImageBuf, Env, EventCtx};
                  
use self::druid::widget::{Align, Image, Flex, Button, FillStrat, Tabs,
                          Label, RadioGroup};

use window_open_file::druid::WidgetExt;

use self::druid_widget_nursery::dropdown::DROPDOWN_SHOW;
use self::druid_widget_nursery::Dropdown;


#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum IOSystemSelection {
    File,
    IO,
}
/// # DropDownState
///
/// State of the application.
#[derive(Data, Clone, Lens)]
struct DropDownState {
    io: IOSystemSelection,
    place: String,
    name: String,
}

/// # WINDOW_TITLE
///
/// Title of the application.

const WINDOW_TITLE: LocalizedString<DropDownState> = LocalizedString::new("Open the file");

/// # build_flex_logo_and_about
///
/// Builds the flex with the rair log and the about button.

fn build_flex_logo_and_about() -> Flex <DropDownState> {
    Flex::column()
        .with_child(build_logo())
        .with_child(Button::new("about"))
}

/// # build_tabs
///
/// Builds tabs to open file, shellcode and projects.

fn build_tabs() -> impl Widget<DropDownState> {
    Tabs::new()
        .with_tab("Open The File", Flex::column()
            .with_child(
                Dropdown::new(
                     Button::new(|f: &IOSystemSelection, _: &Env| format!("{:?}", f)).on_click(
                        |ctx: &mut EventCtx, _, _| ctx.submit_notification(DROPDOWN_SHOW),
                    ),
                    |_, _| {
                        RadioGroup::new(vec![
                            ("file", IOSystemSelection::File),//(IOSystemSelection::File, "file://"),
                            ("io", IOSystemSelection::IO),//(IOSystemSelection::IO, "io://"),
                        ])
                        .fix_size(100., 400.)
                    },
                )
                .align_left()
                .lens(DropDownState::io),
            )
            .with_spacer(100.)
            .with_child(Button::new("Open")))
        .with_tab("Open The Shellcode", Label::new("Proxy settings"))
        .with_tab("Open Project", Label::new("Proxy settings"))
}

/// # build_tabs
///
/// Builds tabs to open file, shellcode and projects.

fn build_windows_content_flex() -> impl Widget<DropDownState> {
    Flex::column()
        .with_child(build_flex_logo_and_about())
        .with_child(build_tabs())
}

/// # build_logo
///
/// Builds the rair logo (currently the odcoder png)

fn build_logo() -> impl Widget<DropDownState> {
    let logo_oddcoder = ImageBuf::from_data(include_bytes!("../data/oddcoder.png")).unwrap();
    let img = Image::new(logo_oddcoder).fill_mode(FillStrat::Fill);
    let centered = Align::horizontal(UnitPoint::TOP, img);
    centered
}

/// # build_windows_to_open_binary
///
/// Builds a label with the text passed as argument.

fn build_windows_to_open_binary() -> WindowDesc<DropDownState> {
    let main_window = WindowDesc::new(build_windows_content_flex())
        .title(WINDOW_TITLE)
        .window_size((400.0, 700.0));
    main_window
}

/// # build_main_gui
///
/// Builds everything.

pub fn build_main_gui() -> Result<(), PlatformError> {
    // create the initial app state
    let initial_state = DropDownState {
        io: IOSystemSelection::File,
        place: "file://".into(),
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
