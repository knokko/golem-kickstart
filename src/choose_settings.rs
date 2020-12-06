use blinds::*;
use mint::Vector2;

pub fn create() -> Settings {
    Settings {
        size: Vector2 {
            x: 1024.0,
            y: 768.0,
        },
        cursor_icon: Some(CursorIcon::Default),
        fullscreen: false,
        icon_path: None,
        multisampling: None,
        vsync: true,
        resizable: true,
        title: "Some Golem App",
    }
}