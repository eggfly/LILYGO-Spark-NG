use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

struct ToolInfo {
    icon: &'static str,
    label: &'static str,
}

const TOOLS: &[ToolInfo] = &[
    ToolInfo { icon: "🔴", label: "Resistor Color" },
    ToolInfo { icon: "🖼", label: "Image Converter" },
    ToolInfo { icon: "🔢", label: "Voltage Divider" },
    ToolInfo { icon: "⏱", label: "RC Time Constant" },
    ToolInfo { icon: "⚡", label: "Ohm's Law" },
    ToolInfo { icon: "⏲", label: "555 Timer" },
    ToolInfo { icon: "📦", label: "SMD Resistor" },
    ToolInfo { icon: "💡", label: "LED Resistor" },
    ToolInfo { icon: "🔋", label: "Battery Life" },
    ToolInfo { icon: "🖥", label: "ESP32 Power" },
    ToolInfo { icon: "🔀", label: "Series/Parallel" },
    ToolInfo { icon: "📐", label: "Circuit Templates" },
];

impl SparkApp {
    pub fn render_embedded_tools(&self) -> impl IntoElement {
        div()
            .id("embedded-tools-page")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_6()
            .overflow_y_scroll()
            .child(page_header("📄", self.i18n.t("tools.title"), self.i18n.t("tools.subtitle")))
            // Tool grid
            .child(
                {
                    let mut grid = div()
                        .flex()
                        .flex_wrap()
                        .gap_2();

                    for (i, tool) in TOOLS.iter().enumerate() {
                        let is_active = i == 0;
                        let mut btn = div()
                            .id(SharedString::from(format!("tool-{}", i)))
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .text_sm()
                            .cursor_pointer();

                        if is_active {
                            btn = btn
                                .bg(rgb(PRIMARY))
                                .text_color(rgb(0xffffff))
                                .shadow_sm();
                        } else {
                            btn = btn
                                .bg(glass_card())
                                .border_1()
                                .border_color(glass_border())
                                .text_color(rgb(TEXT_MUTED))
                                .hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)).text_color(rgb(TEXT_PRIMARY)));
                        }

                        btn = btn
                            .child(tool.icon.to_string())
                            .child(tool.label.to_string());

                        grid = grid.child(btn);
                    }

                    grid
                },
            )
            // Tool content area (placeholder for selected tool)
            .child(
                glass_card_div()
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div().text_color(rgb(TEXT_PRIMARY)).child("🔴 Resistor Color Code Calculator"),
                    )
                    .child(
                        div().text_sm().text_color(rgb(TEXT_MUTED)).child("Select resistor bands to calculate resistance value"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(Self::color_band("Band 1", "Brown", 0x8B4513))
                            .child(Self::color_band("Band 2", "Black", 0x1a1a1a))
                            .child(Self::color_band("Band 3", "Red", 0xdc2626))
                            .child(Self::color_band("Band 4", "Gold", 0xd4a017)),
                    )
                    .child(
                        div()
                            .mt_2()
                            .p_4()
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div().text_2xl().text_color(rgb(GREEN)).child("1.0 kΩ ±5%"),
                            ),
                    ),
            )
    }

    fn color_band(label: &str, color_name: &str, color: u32) -> Div {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(
                div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()),
            )
            .child(
                div()
                    .w(px(40.0))
                    .h(px(60.0))
                    .rounded_md()
                    .bg(rgb(color)),
            )
            .child(
                div().text_xs().text_color(rgb(TEXT_SECONDARY)).child(color_name.to_string()),
            )
    }
}
