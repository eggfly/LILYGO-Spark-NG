use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::app::SparkApp;
use crate::theme::*;

struct SettingItem {
    icon: &'static str,
    name: &'static str,
    description: &'static str,
    control: SettingControl,
}

enum SettingControl {
    Select(&'static [&'static str]),
    Toggle(bool),
    ColorPicker,
    Button(&'static str),
}

const SETTINGS: &[SettingItem] = &[
    SettingItem { icon: "🌐", name: "Language", description: "Display language", control: SettingControl::Select(&["System", "English", "中文简体", "中文繁體", "日本語"]) },
    SettingItem { icon: "🌙", name: "Theme", description: "Appearance mode", control: SettingControl::Select(&["System", "Light", "Dark"]) },
    SettingItem { icon: "🎨", name: "Theme Color", description: "Accent color for the interface", control: SettingControl::ColorPicker },
    SettingItem { icon: "🔗", name: "Link Open Mode", description: "How to open external links", control: SettingControl::Select(&["Internal", "External"]) },
    SettingItem { icon: "✨", name: "Glass Effect", description: "Enable frosted glass UI effect", control: SettingControl::Toggle(true) },
    SettingItem { icon: "🔊", name: "Sound", description: "Play sounds on events", control: SettingControl::Toggle(true) },
    SettingItem { icon: "🎆", name: "Flash Animation", description: "Celebration style after flashing", control: SettingControl::Select(&["Fireworks", "Hacker", "Minimal", "Neon", "Terminal", "Gradient"]) },
    SettingItem { icon: "🔄", name: "Check Update", description: "Check for app updates", control: SettingControl::Button("Check Now") },
    SettingItem { icon: "💾", name: "Download Cache", description: "Manage downloaded firmware files", control: SettingControl::Button("Clear All") },
];

const ACCENT_COLORS: &[(u32, &str)] = &[
    (0x3b82f6, "Blue"),
    (0xf97316, "Orange"),
    (0xf59e0b, "Amber"),
    (0x10b981, "Emerald"),
    (0x06b6d4, "Cyan"),
    (0x0ea5e9, "Sky"),
    (0x8b5cf6, "Violet"),
    (0xf43f5e, "Rose"),
];

impl SparkApp {
    pub fn render_settings(&self) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Tab bar: Settings | Feedback
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_6()
                    .pt_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .bg(hsla(270. / 360., 0.5, 0.5, 0.15))
                            .text_color(rgb(PRIMARY))
                            .text_sm()
                            .cursor_pointer()
                            .child("⚙️ Settings"),
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
                            .child("💬 Feedback"),
                    ),
            )
            // Settings content
            .child(
                div()
                    .id("settings-page")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(
                        {
                            let mut container = div()
                                .max_w(px(640.0))
                                .flex()
                                .flex_col()
                                .gap_3();

                            for setting in SETTINGS {
                                container = container.child(Self::render_setting_item(setting));
                            }

                            container
                        },
                    ),
            )
    }

    fn render_setting_item(item: &SettingItem) -> Div {
        let control = match &item.control {
            SettingControl::Select(options) => {
                let first = options.first().copied().unwrap_or("");
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py(px(6.0))
                    .rounded_lg()
                    .bg(hsla(0., 0., 0., 0.2))
                    .border_1()
                    .border_color(glass_border())
                    .cursor_pointer()
                    .child(div().text_sm().text_color(rgb(TEXT_SECONDARY)).child(first.to_string()))
                    .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("▼"))
            }
            SettingControl::Toggle(enabled) => {
                let (bg, label) = if *enabled {
                    (rgb(PRIMARY), "ON")
                } else {
                    (rgb(TEXT_MUTED), "OFF")
                };
                div()
                    .px_3()
                    .py(px(4.0))
                    .rounded_full()
                    .bg(bg)
                    .text_xs()
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .child(label)
            }
            SettingControl::ColorPicker => {
                let mut row = div().flex().gap_2();
                for &(color, _) in ACCENT_COLORS {
                    row = row.child(
                        div()
                            .w(px(24.0))
                            .h(px(24.0))
                            .rounded_full()
                            .bg(rgb(color))
                            .cursor_pointer()
                            .when(color == PRIMARY, |d: Div| {
                                d.border_2().border_color(rgb(0xffffff))
                            }),
                    );
                }
                row
            }
            SettingControl::Button(label) => {
                div()
                    .px_4()
                    .py(px(6.0))
                    .rounded_lg()
                    .bg(hsla(270. / 360., 0.4, 0.4, 0.2))
                    .text_sm()
                    .text_color(rgb(PRIMARY))
                    .cursor_pointer()
                    .hover(|s| s.bg(hsla(270. / 360., 0.4, 0.4, 0.3)))
                    .child(label.to_string())
            }
        };

        glass_card_div()
            .flex()
            .items_center()
            .justify_between()
            .p_4()
            .hover(|s| s.border_color(glass_border_hover()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(div().child(item.icon.to_string()))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(item.name.to_string()))
                            .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(item.description.to_string())),
                    ),
            )
            .child(control)
    }
}
