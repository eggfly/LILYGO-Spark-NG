use gpui::*;

struct SparkApp {
    count: i32,
}

impl SparkApp {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();
    }
}

impl Render for SparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.count;

        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .bg(rgb(0x1a1a2e))
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(0x00d4ff))
                    .mb_4()
                    .child("LILYGO Spark NT"),
            )
            .child(
                div()
                    .text_color(rgb(0xaaaaaa))
                    .mb_6()
                    .child("A GPUI-based cross-platform desktop application"),
            )
            .child(
                div()
                    .text_color(rgb(0xe94560))
                    .text_xl()
                    .mb_6()
                    .child(format!("Count: {}", count)),
            )
            .child(
                div()
                    .id("increment-btn")
                    .px_8()
                    .py_3()
                    .bg(rgb(0x0f3460))
                    .hover(|s| s.bg(rgb(0x16213e)))
                    .active(|s| s.bg(rgb(0x0a1a30)))
                    .rounded_lg()
                    .cursor_pointer()
                    .text_color(rgb(0xffffff))
                    .child("Click Me")
                    .on_click(cx.listener(Self::increment)),
            )
            .child(
                div()
                    .mt_8()
                    .text_color(rgb(0x555555))
                    .child("Powered by GPUI (Zed's UI Framework)"),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(600.0), px(400.0)),
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some("LILYGO Spark NT".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |_window, cx| {
            cx.new(|_| SparkApp::new())
        })
        .unwrap();
    });
}
