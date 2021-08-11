use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WindowDesc, widget::{Label, TextBox}};

const WINDOW_TITLE: LocalizedString<()> = LocalizedString::new("Guava - Roblox Content Manager");

fn test() -> impl Widget<()> {
    Label::new("Hello Coda & Kisty!")
}

fn main() -> Result<(), PlatformError> {
    let primary_window = WindowDesc::new(test())
        .title(WINDOW_TITLE)
        .window_size((450.0, 450.0));

    AppLauncher::with_window(primary_window).launch(())?;
    Ok(())
}