use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_firmware_lab(&self) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Tab bar
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(glass_border())
                    .child(Self::lab_tab("⚡", "Burner", true))
                    .child(Self::lab_tab("⬇", "Dumper", false))
                    .child(Self::lab_tab("🔬", "Analyzer", false))
                    .child(Self::lab_tab("📋", "Partition Editor", false)),
            )
            // Content area - Burner view (default)
            .child(
                div()
                    .id("firmware-lab-content")
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_4()
                    .overflow_y_scroll()
                    // Controls row
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(Self::control_select("Port", "Select port...", "🔌"))
                            .child(Self::control_select("Tool", "esptool-js", "🛠"))
                            .child(Self::control_select("Chip", "Auto Detect", "💾"))
                            .child(Self::control_select("Baud", "921600", "📡")),
                    )
                    // File area
                    .child(
                        glass_card_div()
                            .p_6()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_3()
                            .min_h(px(120.0))
                            .border_2()
                            .border_color(glass_border())
                            .child(
                                div().text_2xl().child("📂"),
                            )
                            .child(
                                div().text_sm().text_color(rgb(TEXT_MUTED)).child("Drop .bin file here or click to select"),
                            )
                            .child(
                                div().text_xs().text_color(rgb(TEXT_MUTED)).child("Supports: .bin firmware files"),
                            ),
                    )
                    // Flash button
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child("Ready to flash"),
                            )
                            .child(
                                div()
                                    .px_6()
                                    .py(px(10.0))
                                    .rounded_lg()
                                    .bg(rgb(GREEN))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child("⚡ Start Flashing"),
                            ),
                    )
                    // Terminal placeholder
                    .child(
                        div()
                            .flex_1()
                            .min_h(px(200.0))
                            .rounded_lg()
                            .bg(rgb(0x0f172a))
                            .p_4()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x64748b))
                                    .child("$ Terminal output will appear here..."),
                            ),
                    ),
            )
    }

    fn lab_tab(icon: &str, label: &str, active: bool) -> Div {
        let mut tab = div()
            .flex()
            .items_center()
            .gap_2()
            .px_4()
            .py_2()
            .rounded_lg()
            .cursor_pointer()
            .text_sm();

        if active {
            tab = tab
                .bg(hsla(270. / 360., 0.5, 0.5, 0.15))
                .text_color(rgb(PRIMARY));
        } else {
            tab = tab
                .text_color(rgb(TEXT_MUTED))
                .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));
        }

        tab.child(icon.to_string()).child(label.to_string())
    }

    fn control_select(label: &str, value: &str, icon: &str) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()),
            )
            .child(
                glass_card_div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .cursor_pointer()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(div().text_sm().child(icon.to_string()))
                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(value.to_string()))
                    .child(div().flex_1())
                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼")),
            )
    }
}
