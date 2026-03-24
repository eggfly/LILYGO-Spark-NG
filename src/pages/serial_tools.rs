use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_serial_tools(&self) -> impl IntoElement {
        let t_select_port = self.i18n.t("serial.select_port").to_string();
        let t_connect = self.i18n.t("serial.connect").to_string();
        let t_clear = self.i18n.t("serial.clear").to_string();
        let t_send = self.i18n.t("serial.send").to_string();
        let t_type_cmd = self.i18n.t("serial.type_command").to_string();
        let primary = self.primary();

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Toolbar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(glass_border())
                    // Port select
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(glass_card())
                            .border_1()
                            .border_color(glass_border())
                            .cursor_pointer()
                            .hover(|s| s.border_color(glass_border_hover()))
                            .child(div().text_sm().child("🔌"))
                            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(t_select_port))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
                    )
                    // Refresh ports
                    .child(
                        div()
                            .px_2()
                            .py(px(6.0))
                            .rounded_lg()
                            .cursor_pointer()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)).bg(hsla(0., 0., 0.5, 0.1)))
                            .child("🔄"),
                    )
                    // Baud rate
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(glass_card())
                            .border_1()
                            .border_color(glass_border())
                            .cursor_pointer()
                            .hover(|s| s.border_color(glass_border_hover()))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("Baud"))
                            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child("115200"))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
                    )
                    // Connect button
                    .child(
                        div()
                            .px_4()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(rgb(GREEN))
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .child(t_connect),
                    )
                    // Separator
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(20.0))
                            .bg(glass_border()),
                    )
                    // Line ending selector
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(glass_card())
                            .border_1()
                            .border_color(glass_border())
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("NL+CR"))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
                    )
                    .child(div().flex_1())
                    // Timestamp toggle
                    .child(
                        div()
                            .px_2()
                            .py(px(6.0))
                            .rounded_lg()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .cursor_pointer()
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)).bg(hsla(0., 0., 0.5, 0.1)))
                            .child("🕐 Timestamp"),
                    )
                    // Auto-scroll
                    .child(
                        div()
                            .px_2()
                            .py(px(6.0))
                            .rounded_lg()
                            .text_xs()
                            .text_color(rgb(GREEN))
                            .cursor_pointer()
                            .child("↓ Auto-scroll"),
                    )
                    // Clear
                    .child(
                        div()
                            .px_2()
                            .py(px(6.0))
                            .rounded_lg()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)).bg(hsla(0., 0., 0.5, 0.1)))
                            .child(format!("🗑 {}", t_clear)),
                    ),
            )
            // Warnings panel (collapsible, matches Electron)
            .child(
                div()
                    .px_4()
                    .py_2()
                    .bg(hsla(30. / 360., 0.8, 0.2, 0.15))
                    .border_b_1()
                    .border_color(hsla(30. / 360., 0.6, 0.4, 0.2))
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_xs().child("⚠️"))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0xfbbf24))
                            .child("No serial ports detected. Connect a device or install drivers."),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .cursor_pointer()
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                            .child("✕"),
                    ),
            )
            // Terminal area
            .child(
                div()
                    .id("serial-terminal")
                    .flex_1()
                    .bg(rgb(0x0a0a0f))
                    .p_4()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    // Welcome lines
                    .child(Self::terminal_line("system", "Serial Monitor v1.0 — LILYGO Spark NG"))
                    .child(Self::terminal_line("info", "Supported baud rates: 9600, 115200, 230400, 460800, 921600"))
                    .child(Self::terminal_line("info", "Auto-detection: PSRAM errors, Brownout, Guru Meditation, Stack overflow"))
                    .child(Self::terminal_line("info", "Tip: Press Ctrl+L to clear the terminal"))
                    .child(div().h(px(8.0)))
                    .child(Self::terminal_line("dim", "Waiting for connection..."))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(div().text_xs().text_color(rgb(0x22c55e)).child("▊")) // blinking cursor
                    ),
            )
            // Input bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .border_t_1()
                    .border_color(glass_border())
                    .bg(hsla(0., 0., 0., 0.1))
                    .child(
                        div().text_sm().text_color(rgb(0x22c55e)).child("❯"),
                    )
                    .child(
                        div()
                            .flex_1()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.2))
                            .border_1()
                            .border_color(glass_border())
                            .child(
                                div().text_sm().text_color(rgb(TEXT_MUTED)).child(t_type_cmd),
                            ),
                    )
                    .child(
                        div()
                            .px_4()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(rgb(primary))
                            .text_sm()
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .child(t_send),
                    ),
            )
    }

    fn terminal_line(kind: &str, text: &str) -> Div {
        let color = match kind {
            "system" => hsla(140. / 360., 0.7, 0.5, 1.0),
            "info" => hsla(140. / 360., 0.5, 0.5, 0.6),
            "warn" => hsla(45. / 360., 0.9, 0.6, 1.0),
            "error" => hsla(0., 0.8, 0.6, 1.0),
            "dim" => hsla(0., 0., 0.4, 0.5),
            _ => hsla(0., 0., 0.5, 0.6),
        };

        let prefix = match kind {
            "system" => "➤ ",
            "info" => "  ",
            "warn" => "⚠ ",
            "error" => "✗ ",
            _ => "  ",
        };

        div()
            .text_xs()
            .text_color(color)
            .child(format!("{}{}", prefix, text))
    }
}
