use gpui::*;

use crate::pages::Page;
use crate::theme::*;

pub struct SparkApp {
    pub current_page: Page,
}

impl SparkApp {
    pub fn new() -> Self {
        Self {
            current_page: Page::FirmwareCenter,
        }
    }

    pub fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify();
    }
}

impl Render for SparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content: AnyElement = match self.current_page {
            Page::Discovery => self.render_discovery().into_any_element(),
            Page::FirmwareCenter => self.render_firmware_center().into_any_element(),
            Page::FirmwareLab => self.render_firmware_lab().into_any_element(),
            Page::SerialTools => self.render_serial_tools().into_any_element(),
            Page::EmbeddedTools => self.render_embedded_tools().into_any_element(),
            Page::Community => self.render_community().into_any_element(),
            Page::SparkLab => self.render_spark_lab().into_any_element(),
            Page::Settings => self.render_settings().into_any_element(),
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(linear_gradient(
                135.,
                linear_color_stop(hsla(240. / 360., 0.15, 0.10, 1.0), 0.),
                linear_color_stop(hsla(260. / 360., 0.12, 0.12, 1.0), 1.0),
            ))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .overflow_hidden()
                    .child(self.render_sidebar(cx))
                    .child(content),
            )
    }
}
