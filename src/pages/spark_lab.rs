use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

struct SparkItem {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
    status: &'static str, // "done", "planned", "idea"
}

const FLASH_ITEMS: &[SparkItem] = &[
    SparkItem { icon: "📡", name: "WiFi OTA Wireless Flash", description: "Flash firmware wirelessly via WiFi OTA", status: "planned" },
    SparkItem { icon: "🔄", name: "Multi-Device Batch Flash", description: "Flash multiple devices simultaneously", status: "idea" },
    SparkItem { icon: "🔍", name: "Firmware Diff Compare", description: "Compare two firmware binaries side by side", status: "idea" },
    SparkItem { icon: "💾", name: "Firmware Rollback & Backup", description: "Backup and restore firmware versions", status: "planned" },
    SparkItem { icon: "📦", name: "Custom Firmware Repository", description: "Host and manage custom firmware repos", status: "idea" },
];

const DEVICE_ITEMS: &[SparkItem] = &[
    SparkItem { icon: "📊", name: "Serial Plotter", description: "Visualize serial data in real-time charts", status: "planned" },
    SparkItem { icon: "🔌", name: "GPIO Live Monitor", description: "Monitor GPIO pin states in real-time", status: "idea" },
    SparkItem { icon: "🔬", name: "I2C/SPI Device Scanner", description: "Scan and identify I2C/SPI devices on bus", status: "idea" },
    SparkItem { icon: "📶", name: "Bluetooth Debug Assistant", description: "Debug BLE connections and services", status: "idea" },
    SparkItem { icon: "🌐", name: "MQTT Test Client", description: "Test MQTT publish/subscribe messaging", status: "idea" },
];

const CALCULATOR_ITEMS: &[SparkItem] = &[
    SparkItem { icon: "⚡", name: "Ohm's Law Calculator", description: "Calculate voltage, current, resistance, power", status: "done" },
    SparkItem { icon: "⏲", name: "555 Timer Calculator", description: "Calculate 555 timer astable/monostable", status: "done" },
    SparkItem { icon: "🔋", name: "Battery Life Calculator", description: "Estimate battery runtime from capacity", status: "done" },
    SparkItem { icon: "🖥", name: "ESP32 Power Mode Calculator", description: "Calculate power for different ESP32 modes", status: "done" },
    SparkItem { icon: "🔀", name: "Series/Parallel R/C Calculator", description: "Calculate combined resistance/capacitance", status: "done" },
    SparkItem { icon: "📐", name: "Interactive Circuit Schematic", description: "View and edit basic circuit schematics", status: "done" },
    SparkItem { icon: "💡", name: "LED Current Limiting Resistor", description: "Calculate LED series resistor value", status: "done" },
    SparkItem { icon: "📦", name: "SMD Resistor Calculator", description: "Decode SMD resistor marking codes", status: "done" },
    SparkItem { icon: "🔴", name: "Resistor Color Code Calculator", description: "Read resistor color bands", status: "done" },
    SparkItem { icon: "⏱", name: "RC Time Constant Calculator", description: "Calculate RC circuit time constants", status: "done" },
    SparkItem { icon: "🔢", name: "Voltage Divider Calculator", description: "Calculate voltage divider output", status: "done" },
    SparkItem { icon: "📡", name: "Power/dBm Converter", description: "Convert between watts and dBm", status: "planned" },
    SparkItem { icon: "📻", name: "Antenna Impedance Matching", description: "Calculate antenna matching networks", status: "idea" },
    SparkItem { icon: "🔲", name: "PCB Trace Width Calculator", description: "Calculate trace width for current capacity", status: "idea" },
];

const CREATIVE_ITEMS: &[SparkItem] = &[
    SparkItem { icon: "🎲", name: "3D Board Preview", description: "Preview development boards in 3D", status: "idea" },
    SparkItem { icon: "🤖", name: "AI Firmware Recommender", description: "AI-powered firmware suggestions", status: "idea" },
    SparkItem { icon: "⭐", name: "Community Firmware Ratings", description: "Rate and review community firmware", status: "idea" },
    SparkItem { icon: "✅", name: "Hardware Compatibility Check", description: "Check firmware-hardware compatibility", status: "planned" },
    SparkItem { icon: "📄", name: "Project Template Generator", description: "Generate project scaffolding from templates", status: "idea" },
];

fn all_items() -> impl Iterator<Item = &'static SparkItem> {
    FLASH_ITEMS.iter()
        .chain(DEVICE_ITEMS.iter())
        .chain(CALCULATOR_ITEMS.iter())
        .chain(CREATIVE_ITEMS.iter())
}

impl SparkApp {
    pub fn render_spark_lab(&self) -> impl IntoElement {
        let done = all_items().filter(|i| i.status == "done").count();
        let planned = all_items().filter(|i| i.status == "planned").count();
        let ideas = all_items().filter(|i| i.status == "idea").count();
        let total = done + planned + ideas;
        let pct = if total > 0 { done * 100 / total } else { 0 };

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Tab header
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(glass_border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .bg(hsla(270. / 360., 0.5, 0.5, 0.15))
                            .text_color(rgb(self.primary()))
                            .text_sm()
                            .cursor_pointer()
                            .child(format!("✨ {}", self.i18n.t("sparklab.sparkling_list"))),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .text_color(rgb(TEXT_MUTED))
                            .text_sm()
                            .cursor_pointer()
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)))
                            .child(format!("📖 {}", self.i18n.t("sparklab.guide"))),
                    ),
            )
            // Content
            .child(div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_6()
            .id("spark-lab-page")
            .overflow_y_scroll()
            .child(page_header_with_primary("🧪", self.i18n.t("sparklab.title"), self.i18n.t("sparklab.subtitle"), self.primary()))
            // Stats row
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Self::status_pill("✅ Done", done, GREEN))
                    .child(Self::status_pill("🔜 Planned", planned, 0x3b82f6))
                    .child(Self::status_pill("💡 Sparks", ideas, AMBER)),
            )
            // Progress bar
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .h(px(6.0))
                            .w_full()
                            .rounded_full()
                            .bg(hsla(0., 0., 0., 0.2))
                            .child(
                                div()
                                    .h_full()
                                    .rounded_full()
                                    .bg(linear_gradient(
                                        90.,
                                        linear_color_stop(rgb(GREEN), 0.),
                                        linear_color_stop(rgb(0x059669), 1.),
                                    ))
                                    .w(px(pct as f32 / 100.0 * 500.0)),
                            ),
                    )
                    .child(
                        div().text_xs().text_color(rgb(TEXT_MUTED)).child(format!("{}% complete ({}/{})", pct, done, total)),
                    ),
            )
            // 4 Categories
            .child(Self::spark_category("🔧 Flash & Firmware Management", FLASH_ITEMS))
            .child(Self::spark_category("🔌 Device Interaction", DEVICE_ITEMS))
            .child(Self::spark_category("📐 Embedded Calculators", CALCULATOR_ITEMS))
            .child(Self::spark_category("🎨 Creative & Differentiation", CREATIVE_ITEMS)),
            )
    }

    fn status_pill(label: &str, count: usize, color: u32) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .px_3()
            .py_1()
            .rounded_full()
            .bg(hsla(0., 0., 0., 0.2))
            .text_sm()
            .text_color(rgb(color))
            .child(format!("{} {}", label, count))
    }

    fn spark_category(title: &str, items: &[SparkItem]) -> Div {
        let mut card = glass_card_div()
            .flex()
            .flex_col()
            .p_4()
            .gap_3();

        card = card.child(
            div().text_color(rgb(TEXT_PRIMARY)).child(title.to_string()),
        );

        for item in items {
            let (status_text, status_color) = match item.status {
                "done" => ("✅ Done", GREEN),
                "planned" => ("🔜 Planned", 0x3b82f6),
                _ => ("💡 Spark", AMBER),
            };

            card = card.child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .py_2()
                    .child(
                        div()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded_lg()
                            .bg(hsla(270. / 360., 0.3, 0.3, 0.2))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(item.icon.to_string()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(item.name.to_string()),
                            )
                            .child(
                                div().text_xs().text_color(rgb(TEXT_MUTED)).child(item.description.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .px_2()
                            .py(px(2.0))
                            .rounded_md()
                            .text_color(rgb(status_color))
                            .child(status_text.to_string()),
                    ),
            );
        }

        card
    }
}
