#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;

// ─── Translucent color helpers (glass-like) ───
fn glass_card() -> Hsla {
    hsla(230. / 360., 0.25, 0.15, 0.55)
}
fn glass_sidebar() -> Hsla {
    hsla(240. / 360., 0.35, 0.08, 0.70)
}
fn glass_border() -> Hsla {
    hsla(230. / 360., 0.3, 0.4, 0.25)
}
fn glass_border_hover() -> Hsla {
    hsla(195. / 360., 0.8, 0.5, 0.4)
}

const ACCENT: u32 = 0x00d4ff;
const ACCENT_RED: u32 = 0xe94560;
const TEXT_PRIMARY: u32 = 0xffffff;
const TEXT_SECONDARY: u32 = 0xb0b0c0;
const TEXT_MUTED: u32 = 0x666680;

fn card_shadow() -> BoxShadow {
    BoxShadow {
        color: hsla(0., 0., 0., 0.4),
        offset: point(px(0.), px(4.)),
        blur_radius: px(16.),
        spread_radius: px(0.),
    }
}

fn btn_glow() -> BoxShadow {
    BoxShadow {
        color: hsla(350. / 360., 0.8, 0.45, 0.5),
        offset: point(px(0.), px(0.)),
        blur_radius: px(24.),
        spread_radius: px(2.),
    }
}

// ─── Navigation pages ───
#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Devices,
    Firmware,
    Settings,
}

impl Page {
    fn label(&self) -> &'static str {
        match self {
            Page::Home => "Home",
            Page::Devices => "Devices",
            Page::Firmware => "Firmware",
            Page::Settings => "Settings",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Page::Home => "⚡",
            Page::Devices => "📱",
            Page::Firmware => "💾",
            Page::Settings => "⚙️",
        }
    }
}

// ─── Device info ───
struct DeviceInfo {
    name: &'static str,
    mcu: &'static str,
    status: &'static str,
    status_color: u32,
}

const DEVICES: &[DeviceInfo] = &[
    DeviceInfo {
        name: "T-Display S3",
        mcu: "ESP32-S3",
        status: "Connected",
        status_color: 0x00e676,
    },
    DeviceInfo {
        name: "T-Display S3 AMOLED",
        mcu: "ESP32-S3",
        status: "Disconnected",
        status_color: ACCENT_RED,
    },
    DeviceInfo {
        name: "T-Deck",
        mcu: "ESP32-S3",
        status: "Disconnected",
        status_color: ACCENT_RED,
    },
    DeviceInfo {
        name: "T-Watch S3",
        mcu: "ESP32-S3",
        status: "Disconnected",
        status_color: ACCENT_RED,
    },
];

// ─── App state ───
struct SparkApp {
    current_page: Page,
    flash_count: i32,
    sidebar_collapsed: bool,
}

impl SparkApp {
    fn new() -> Self {
        Self {
            current_page: Page::Home,
            flash_count: 0,
            sidebar_collapsed: false,
        }
    }

    fn navigate(&mut self, page: Page, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify();
    }

    fn flash_firmware(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.flash_count += 1;
        cx.notify();
    }

    fn toggle_sidebar(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        cx.notify();
    }

    // ─── Glass card wrapper ───
    fn glass_card() -> Div {
        div()
            .rounded_xl()
            .bg(glass_card())
            .border_1()
            .border_color(glass_border())
            .shadow(vec![card_shadow()])
    }

    // ─── Sidebar ───
    fn render_sidebar(&mut self, cx: &mut Context<Self>) -> Div {
        let current = self.current_page;
        let collapsed = self.sidebar_collapsed;
        let width = if collapsed { px(60.0) } else { px(210.0) };

        let mut sidebar = div()
            .w(width)
            .h_full()
            .flex()
            .flex_col()
            .bg(glass_sidebar())
            .border_r_1()
            .border_color(glass_border());

        // Logo area with gradient accent
        sidebar = sidebar.child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .h(px(60.0))
                .bg(linear_gradient(
                    180.,
                    linear_color_stop(hsla(195. / 360., 0.8, 0.5, 0.15), 0.),
                    linear_color_stop(hsla(270. / 360., 0.6, 0.4, 0.05), 1.),
                ))
                .border_b_1()
                .border_color(glass_border())
                .child(
                    div()
                        .text_color(rgb(ACCENT))
                        .child(if collapsed { "⚡" } else { "⚡ Spark NT" }),
                ),
        );

        // Nav items
        for page in [Page::Home, Page::Devices, Page::Firmware, Page::Settings] {
            let is_active = current == page;
            let label = if collapsed {
                page.icon().to_string()
            } else {
                format!("{}  {}", page.icon(), page.label())
            };

            let mut item = div()
                .id(SharedString::from(format!("nav-{}", page.label())))
                .mx(px(8.0))
                .mt(px(4.0))
                .px(px(12.0))
                .py(px(10.0))
                .rounded_lg()
                .cursor_pointer()
                .hover(|s| s.bg(hsla(230. / 360., 0.3, 0.3, 0.3)))
                .child(label)
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.navigate(page, cx);
                }));

            if is_active {
                item = item
                    .bg(linear_gradient(
                        90.,
                        linear_color_stop(hsla(195. / 360., 0.8, 0.5, 0.2), 0.),
                        linear_color_stop(hsla(270. / 360., 0.6, 0.5, 0.1), 1.),
                    ))
                    .text_color(rgb(ACCENT))
                    .border_l_2()
                    .border_color(rgb(ACCENT))
                    .shadow(vec![BoxShadow {
                        color: hsla(195. / 360., 1.0, 0.5, 0.15),
                        offset: point(px(0.), px(0.)),
                        blur_radius: px(12.),
                        spread_radius: px(0.),
                    }]);
            } else {
                item = item.text_color(rgb(TEXT_SECONDARY));
            }

            sidebar = sidebar.child(item);
        }

        sidebar = sidebar.child(div().flex_1());

        // Collapse toggle
        sidebar = sidebar.child(
            div()
                .id("toggle-sidebar")
                .flex()
                .justify_center()
                .py_3()
                .border_t_1()
                .border_color(glass_border())
                .cursor_pointer()
                .text_color(rgb(TEXT_MUTED))
                .hover(|s| s.text_color(rgb(ACCENT)))
                .child(if collapsed { "▶" } else { "◀ Collapse" })
                .on_click(cx.listener(Self::toggle_sidebar)),
        );

        sidebar
    }

    // ─── Status bar ───
    fn render_status_bar(&self) -> Div {
        div()
            .w_full()
            .h(px(30.0))
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .bg(hsla(240. / 360., 0.3, 0.06, 0.8))
            .border_t_1()
            .border_color(glass_border())
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x00e676))
                            .child("● 1 device connected"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .child(format!("Flashed: {} times", self.flash_count)),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child("LILYGO Spark NT v0.1.0 | GPUI"),
            )
    }

    // ─── Home page ───
    fn render_home(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_6()
            .overflow_hidden()
            // Welcome header with gradient text simulation
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("Welcome to LILYGO Spark NT"),
                    )
                    .child(
                        div()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child("Flash firmware to your LILYGO devices with ease"),
                    ),
            )
            // Stats cards row
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(Self::stat_card("Devices", "4", "Total supported", ACCENT))
                    .child(Self::stat_card(
                        "Connected",
                        "1",
                        "Ready to flash",
                        0x00e676,
                    ))
                    .child(Self::stat_card(
                        "Firmwares",
                        "12",
                        "Available",
                        0xffa726,
                    ))
                    .child(Self::stat_card(
                        "Flashed",
                        &self.flash_count.to_string(),
                        "This session",
                        ACCENT_RED,
                    )),
            )
            // Quick action card
            .child(
                Self::glass_card()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .p_5()
                    .child(
                        div()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("Quick Flash"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_SECONDARY))
                            .child(
                                "T-Display S3 is connected. Flash the latest firmware?",
                            ),
                    )
                    .child(
                        div()
                            .id("flash-btn")
                            .mt_2()
                            .px_6()
                            .py(px(10.0))
                            .w(px(200.0))
                            .flex()
                            .justify_center()
                            .rounded_lg()
                            .bg(linear_gradient(
                                135.,
                                linear_color_stop(rgb(ACCENT_RED), 0.),
                                linear_color_stop(rgb(0xc040a0), 1.),
                            ))
                            .shadow(vec![btn_glow()])
                            .hover(|s| s.opacity(0.85))
                            .active(|s| s.opacity(0.7))
                            .cursor_pointer()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("⚡ Flash Firmware")
                            .on_click(cx.listener(Self::flash_firmware)),
                    ),
            )
    }

    fn stat_card(title: &str, value: &str, subtitle: &str, accent: u32) -> Div {
        let accent_hsla = {
            let r = ((accent >> 16) & 0xff) as f32 / 255.;
            let g = ((accent >> 8) & 0xff) as f32 / 255.;
            let b = (accent & 0xff) as f32 / 255.;
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let l = (max + min) / 2.;
            let s = if max == min {
                0.
            } else if l < 0.5 {
                (max - min) / (max + min)
            } else {
                (max - min) / (2. - max - min)
            };
            let h = if max == min {
                0.
            } else if max == r {
                ((g - b) / (max - min) + if g < b { 6. } else { 0. }) / 6.
            } else if max == g {
                ((b - r) / (max - min) + 2.) / 6.
            } else {
                ((r - g) / (max - min) + 4.) / 6.
            };
            hsla(h, s, l, 0.3)
        };

        Self::glass_card()
            .flex_1()
            .flex()
            .flex_col()
            .gap_1()
            .p_4()
            .hover(|s| s.border_color(glass_border_hover()))
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child(title.to_string()),
            )
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(accent))
                    .shadow(vec![BoxShadow {
                        color: accent_hsla,
                        offset: point(px(0.), px(0.)),
                        blur_radius: px(0.),
                        spread_radius: px(0.),
                    }])
                    .child(value.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(subtitle.to_string()),
            )
    }

    // ─── Devices page ───
    fn render_devices(&self) -> Div {
        let mut page = div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Devices"),
            )
            .child(
                div()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child("Manage your LILYGO development boards"),
            );

        for device in DEVICES {
            let is_connected = device.status == "Connected";
            let mut card = Self::glass_card()
                .flex()
                .items_center()
                .justify_between()
                .p_4()
                .hover(|s| s.border_color(glass_border_hover()));

            if is_connected {
                card = card.shadow(vec![
                    card_shadow(),
                    BoxShadow {
                        color: hsla(140. / 360., 0.8, 0.45, 0.15),
                        offset: point(px(0.), px(0.)),
                        blur_radius: px(16.),
                        spread_radius: px(0.),
                    },
                ]);
            }

            page = page.child(
                card.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child(device.name.to_string()),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_MUTED))
                                .child(format!("MCU: {}", device.mcu)),
                        ),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(device.status_color))
                        .child(format!("● {}", device.status)),
                ),
            );
        }

        page
    }

    // ─── Firmware page ───
    fn render_firmware(&self) -> Div {
        let firmwares = [
            ("Factory Test", "v1.0.0", "2024-12-01", "Official"),
            ("LVGL Demo", "v9.2.0", "2025-01-15", "Community"),
            ("MicroPython", "v1.23.0", "2025-02-20", "Official"),
            ("Arduino Blink", "v1.0.0", "2024-11-10", "Example"),
        ];

        let mut page = div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Firmware Library"),
            )
            .child(
                div()
                    .text_color(rgb(TEXT_SECONDARY))
                    .child("Browse and flash available firmware images"),
            );

        for (name, version, date, source) in firmwares {
            let (badge_color, badge_bg) = match source {
                "Official" => (ACCENT, hsla(195. / 360., 0.8, 0.5, 0.15)),
                "Community" => (0xffa726, hsla(35. / 360., 0.8, 0.5, 0.15)),
                _ => (TEXT_MUTED, hsla(0., 0., 0.3, 0.15)),
            };

            page = page.child(
                Self::glass_card()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_color(rgb(TEXT_PRIMARY))
                                            .child(name.to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .px_2()
                                            .py(px(2.0))
                                            .rounded_md()
                                            .bg(badge_bg)
                                            .text_color(rgb(badge_color))
                                            .child(source.to_string()),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(format!("{} · {}", version, date)),
                            ),
                    ),
            );
        }

        page
    }

    // ─── Settings page ───
    fn render_settings(&self) -> Div {
        let settings = [
            (
                "Auto-connect",
                "Automatically connect to devices on startup",
                true,
            ),
            (
                "Dark mode",
                "Use dark theme (currently the only theme)",
                true,
            ),
            (
                "Check updates",
                "Check for firmware updates on launch",
                false,
            ),
            (
                "Verbose logging",
                "Show detailed flash progress logs",
                false,
            ),
        ];

        let mut page = div()
            .flex_1()
            .flex()
            .flex_col()
            .p_6()
            .gap_4()
            .overflow_hidden()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Settings"),
            );

        for (name, desc, enabled) in settings {
            let (toggle_bg, toggle_color) = if enabled {
                (hsla(195. / 360., 0.8, 0.5, 0.2), rgb(ACCENT))
            } else {
                (hsla(0., 0., 0.3, 0.2), rgb(TEXT_MUTED))
            };

            page = page.child(
                Self::glass_card()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .hover(|s| s.border_color(glass_border_hover()))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_color(rgb(TEXT_PRIMARY))
                                    .child(name.to_string()),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(desc.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .text_sm()
                            .text_color(toggle_color)
                            .bg(toggle_bg)
                            .border_1()
                            .border_color(glass_border())
                            .child(if enabled { "ON" } else { "OFF" }),
                    ),
            );
        }

        // About section
        page = page.child(
            Self::glass_card()
                .mt_4()
                .p_4()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_color(rgb(TEXT_PRIMARY))
                        .child("About"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child("LILYGO Spark NT v0.1.0"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(TEXT_MUTED))
                        .child("Built with GPUI (Zed's UI Framework)"),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(ACCENT))
                        .child("github.com/eggfly/LILYGO-Spark-NT"),
                ),
        );

        page
    }
}

impl Render for SparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.current_page {
            Page::Home => self.render_home(cx),
            Page::Devices => self.render_devices(),
            Page::Firmware => self.render_firmware(),
            Page::Settings => self.render_settings(),
        };

        // Outer shell: gradient mesh background simulating Electron's glass-mesh
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(linear_gradient(
                135.,
                linear_color_stop(hsla(240. / 360., 0.4, 0.08, 1.0), 0.),
                linear_color_stop(hsla(260. / 360., 0.3, 0.10, 1.0), 1.0),
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
            .child(self.render_status_bar())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);

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
                    width: px(640.0),
                    height: px(400.0),
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_| SparkApp::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
