use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_discovery(&self) -> impl IntoElement {
        div()
            .id("discovery-page")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_6()
            .overflow_y_scroll()
            .child(page_header("📰", "Spark Discovery", "Explore the latest embedded community news"))
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .child(Self::news_card("Hackaday", "New ESP32-S3 Projects Roundup", "A collection of the latest projects using ESP32-S3...", "2025-03-20", 0x1a1a2e))
                    .child(Self::news_card("CNX Software", "LILYGO T-Display S3 AMOLED Review", "Hands-on review of the T-Display S3 AMOLED board...", "2025-03-18", 0x1e40af))
                    .child(Self::news_card("Adafruit", "CircuitPython 9.0 Released", "Major update with new board support including ESP32...", "2025-03-15", 0xbe185d))
                    .child(Self::news_card("Reddit", "ESP32 Home Automation Guide", "Complete guide for building smart home with ESP32...", "2025-03-12", 0xea580c))
                    .child(Self::news_card("GitHub", "MicroPython v1.23 Highlights", "What's new in the latest MicroPython release...", "2025-03-10", 0x374151))
                    .child(Self::news_card("Hackaday", "Building a LoRa Mesh Network", "Step by step guide to creating a LoRa mesh...", "2025-03-08", 0x1a1a2e))
            )
    }

    fn news_card(source: &str, title: &str, summary: &str, date: &str, source_color: u32) -> Div {
        glass_card_div()
            .w(px(280.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                // Image placeholder area
                div()
                    .h(px(140.0))
                    .w_full()
                    .bg(hsla(220. / 360., 0.15, 0.15, 0.5))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div().text_color(rgb(TEXT_MUTED)).child("📷"),
                    )
                    // Source badge
                    .child(
                        div()
                            .absolute()
                            .top_2()
                            .left_2()
                            .px_2()
                            .py(px(2.0))
                            .rounded_md()
                            .bg(rgb(source_color))
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child(source.to_string()),
                    ),
            )
            .child(
                div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child(title.to_string()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .child(summary.to_string()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .child(format!("📅 {}", date)),
                    ),
            )
    }
}
