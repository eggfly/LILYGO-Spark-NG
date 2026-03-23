#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod pages;
mod sidebar;
mod theme;

use app::SparkApp;
use gpui::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1100.0), px(700.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Blurred,
                titlebar: Some(TitlebarOptions {
                    title: Some("LILYGO Spark NT".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_min_size: Some(Size {
                    width: px(800.0),
                    height: px(500.0),
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| SparkApp::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
