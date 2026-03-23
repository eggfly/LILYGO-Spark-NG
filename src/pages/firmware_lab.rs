use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

impl SparkApp {
    pub fn render_firmware_lab(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let t_burner = self.i18n.t("lab.burner").to_string();
        let t_dumper = self.i18n.t("lab.dumper").to_string();
        let t_analyzer = self.i18n.t("lab.analyzer").to_string();
        let t_partition = self.i18n.t("lab.partition_editor").to_string();
        let t_start_flash = self.i18n.t("lab.start_flash").to_string();
        let t_ready = self.i18n.t("lab.ready").to_string();
        let t_drop_file = self.i18n.t("lab.drop_file").to_string();
        let active_tab = self.active_lab_tab;
        let primary = self.primary();

        let tabs = [
            ("⚡", t_burner),
            ("⬇", t_dumper),
            ("🔬", t_analyzer),
            ("📋", t_partition),
        ];

        let mut tab_bar = div()
            .flex()
            .items_center()
            .gap_1()
            .px_4()
            .py_2()
            .border_b_1()
            .border_color(glass_border());

        for (i, (icon, label)) in tabs.iter().enumerate() {
            let is_active = active_tab == i;
            let mut tab = div()
                .id(SharedString::from(format!("lab-tab-{}", i)))
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .py_2()
                .rounded_lg()
                .cursor_pointer()
                .text_sm();

            if is_active {
                tab = tab
                    .bg(self.primary_alpha(0.15))
                    .text_color(rgb(primary));
            } else {
                tab = tab
                    .text_color(rgb(TEXT_MUTED))
                    .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));
            }

            tab = tab
                .child(icon.to_string())
                .child(label.clone())
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.active_lab_tab = i;
                    cx.notify();
                }));

            tab_bar = tab_bar.child(tab);
        }

        let content: AnyElement = match active_tab {
            0 => self.render_burner_tab(&t_start_flash, &t_ready, &t_drop_file),
            1 => self.render_dumper_tab(),
            2 => self.render_analyzer_tab(),
            3 => self.render_partition_tab(),
            _ => self.render_burner_tab(&t_start_flash, &t_ready, &t_drop_file),
        };

        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(tab_bar)
            .child(content)
    }

    fn render_burner_tab(&self, t_start_flash: &str, t_ready: &str, t_drop_file: &str) -> AnyElement {
        let primary = self.primary();
        div()
            .id("burner-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            // Mode toggle row
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .p(px(2.0))
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.15))
                            .child(
                                div()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .bg(rgb(primary))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .child("Basic"),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py(px(6.0))
                                    .rounded_md()
                                    .text_xs()
                                    .text_color(rgb(TEXT_MUTED))
                                    .cursor_pointer()
                                    .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                                    .child("Advanced"),
                            ),
                    )
                    .child(
                        div()
                            .px_3()
                            .py(px(6.0))
                            .rounded_lg()
                            .bg(hsla(0., 0., 0., 0.15))
                            .border_1()
                            .border_color(glass_border())
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .cursor_pointer()
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                            .child("👁 Preview"),
                    ),
            )
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
                    .child(div().text_2xl().child("📂"))
                    .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(t_drop_file.to_string()))
                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("Supports: .bin firmware files")),
            )
            // Flash button
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(div().flex_1().text_sm().text_color(rgb(TEXT_MUTED)).child(t_ready.to_string()))
                    .child(
                        div()
                            .px_6()
                            .py(px(10.0))
                            .rounded_lg()
                            .bg(rgb(GREEN))
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .child(format!("⚡ {}", t_start_flash)),
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
            )
            .into_any_element()
    }

    fn render_dumper_tab(&self) -> AnyElement {
        div()
            .id("dumper-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            // Controls
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(Self::control_select("Port", "Select port...", "🔌"))
                    .child(Self::control_select("Baud", "921600", "📡")),
            )
            // Device info card
            .child(
                glass_card_div()
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(div().text_color(rgb(TEXT_PRIMARY)).child("Device Information"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_3()
                            .child(Self::info_chip("Chip", "—"))
                            .child(Self::info_chip("Flash ID", "—"))
                            .child(Self::info_chip("MAC", "—"))
                            .child(Self::info_chip("Crystal", "—")),
                    ),
            )
            // Dump settings
            .child(
                glass_card_div()
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(div().text_color(rgb(TEXT_PRIMARY)).child("Dump Settings"))
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(Self::control_select("Start Address", "0x0", "📍"))
                            .child(Self::control_select("Size", "4 MB", "📏")),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .child(
                                div()
                                    .px_6()
                                    .py(px(10.0))
                                    .rounded_lg()
                                    .bg(rgb(GREEN))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child("⬇ Start Dump"),
                            ),
                    ),
            )
            .into_any_element()
    }

    fn render_analyzer_tab(&self) -> AnyElement {
        div()
            .id("analyzer-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            // Drop zone
            .child(
                glass_card_div()
                    .p_8()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_3()
                    .min_h(px(160.0))
                    .border_2()
                    .border_color(glass_border())
                    .child(div().text_3xl().child("🔬"))
                    .child(div().text_color(rgb(TEXT_PRIMARY)).child("Firmware Analyzer"))
                    .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child("Drop a .bin file to analyze firmware contents"))
                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("Detects chip type, partitions, build info, and more")),
            )
            // Results area (empty state)
            .child(
                glass_card_div()
                    .p_6()
                    .flex()
                    .items_center()
                    .justify_center()
                    .min_h(px(100.0))
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .child("Analysis results will appear here..."),
                    ),
            )
            .into_any_element()
    }

    fn render_partition_tab(&self) -> AnyElement {
        div()
            .id("partition-content")
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(div().text_color(rgb(TEXT_PRIMARY)).child("Partition Table Editor"))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_4()
                                    .py(px(6.0))
                                    .rounded_lg()
                                    .bg(hsla(0., 0., 0., 0.15))
                                    .border_1()
                                    .border_color(glass_border())
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0., 0., 0., 0.25)))
                                    .child("📂 Import"),
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py(px(6.0))
                                    .rounded_lg()
                                    .bg(hsla(0., 0., 0., 0.15))
                                    .border_1()
                                    .border_color(glass_border())
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0., 0., 0., 0.25)))
                                    .child("💾 Export"),
                            ),
                    ),
            )
            // Partition table (mock)
            .child(
                glass_card_div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Header row
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .child(div().w(px(100.0)).child("Name"))
                            .child(div().w(px(80.0)).child("Type"))
                            .child(div().w(px(100.0)).child("Address"))
                            .child(div().w(px(80.0)).child("Size"))
                            .child(div().flex_1().child("Flags")),
                    )
                    // Sample rows
                    .child(Self::partition_row("nvs", "data", "0x9000", "20 KB", ""))
                    .child(Self::partition_row("otadata", "data", "0xe000", "8 KB", ""))
                    .child(Self::partition_row("app0", "app", "0x10000", "1.2 MB", ""))
                    .child(Self::partition_row("spiffs", "data", "0x290000", "1.4 MB", "")),
            )
            .into_any_element()
    }

    fn info_chip(label: &str, value: &str) -> Div {
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .px_3()
            .py_2()
            .rounded_lg()
            .bg(hsla(0., 0., 0., 0.1))
            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(label.to_string()))
            .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(value.to_string()))
    }

    fn partition_row(name: &str, ptype: &str, addr: &str, size: &str, flags: &str) -> Div {
        div()
            .flex()
            .gap_2()
            .px_2()
            .py(px(6.0))
            .rounded_md()
            .text_sm()
            .hover(|s| s.bg(hsla(0., 0., 0., 0.1)))
            .child(div().w(px(100.0)).text_color(rgb(TEXT_PRIMARY)).child(name.to_string()))
            .child(div().w(px(80.0)).text_color(rgb(TEXT_MUTED)).child(ptype.to_string()))
            .child(div().w(px(100.0)).text_color(rgb(TEXT_SECONDARY)).child(addr.to_string()))
            .child(div().w(px(80.0)).text_color(rgb(TEXT_SECONDARY)).child(size.to_string()))
            .child(div().flex_1().text_color(rgb(TEXT_MUTED)).child(flags.to_string()))
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
