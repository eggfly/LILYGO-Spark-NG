use gpui::*;

// --- Color theme ---
const BG_PRIMARY: u32 = 0x0f0f1a;
const BG_SIDEBAR: u32 = 0x151525;
const BG_PANEL: u32 = 0x1a1a2e;
const BG_STATUS: u32 = 0x0a0a15;
const BG_BUTTON: u32 = 0x0f3460;
const BG_BUTTON_HOVER: u32 = 0x16213e;
const BG_BUTTON_ACTIVE: u32 = 0x0a1a30;
const BG_CARD: u32 = 0x16213e;
const ACCENT: u32 = 0x00d4ff;
const ACCENT2: u32 = 0xe94560;
const TEXT_PRIMARY: u32 = 0xeaeaea;
const TEXT_SECONDARY: u32 = 0x8888aa;
const TEXT_DIM: u32 = 0x555570;
const BORDER: u32 = 0x2a2a40;
const GREEN: u32 = 0x4ade80;
const YELLOW: u32 = 0xfbbf24;

#[derive(Clone, Copy, PartialEq)]
enum NavItem {
    Dashboard,
    Devices,
    Firmware,
    Settings,
}

impl NavItem {
    fn label(&self) -> &'static str {
        match self {
            NavItem::Dashboard => "Dashboard",
            NavItem::Devices => "Devices",
            NavItem::Firmware => "Firmware",
            NavItem::Settings => "Settings",
        }
    }

    fn icon_char(&self) -> &'static str {
        match self {
            NavItem::Dashboard => "◉",
            NavItem::Devices => "⊞",
            NavItem::Firmware => "⬡",
            NavItem::Settings => "⚙",
        }
    }
}

struct SparkApp {
    active_nav: NavItem,
    click_count: i32,
    devices: Vec<DeviceInfo>,
    log_messages: Vec<String>,
}

#[derive(Clone)]
struct DeviceInfo {
    name: &'static str,
    mcu: &'static str,
    status: &'static str,
    firmware: &'static str,
}

impl SparkApp {
    fn new() -> Self {
        Self {
            active_nav: NavItem::Dashboard,
            click_count: 0,
            devices: vec![
                DeviceInfo {
                    name: "T-Display S3",
                    mcu: "ESP32-S3",
                    status: "Online",
                    firmware: "v2.1.0",
                },
                DeviceInfo {
                    name: "T-Display AMOLED",
                    mcu: "ESP32-S3",
                    status: "Online",
                    firmware: "v1.3.2",
                },
                DeviceInfo {
                    name: "T-Deck",
                    mcu: "ESP32-S3",
                    status: "Offline",
                    firmware: "v1.0.5",
                },
                DeviceInfo {
                    name: "T-Watch S3",
                    mcu: "ESP32-S3",
                    status: "Online",
                    firmware: "v3.0.1",
                },
            ],
            log_messages: vec![
                "[INFO]  Application started".into(),
                "[INFO]  GPUI renderer initialized".into(),
                "[INFO]  Device manager ready".into(),
            ],
        }
    }

    fn navigate(&mut self, item: NavItem, cx: &mut Context<Self>) {
        self.active_nav = item;
        self.log_messages
            .push(format!("[NAV]   Switched to {}", item.label()));
        cx.notify();
    }

    fn increment(&mut self, cx: &mut Context<Self>) {
        self.click_count += 1;
        self.log_messages
            .push(format!("[EVENT] Button clicked (count: {})", self.click_count));
        cx.notify();
    }

    fn render_sidebar(&mut self, cx: &mut Context<Self>) -> Div {
        let active = self.active_nav;
        let nav_items = [
            NavItem::Dashboard,
            NavItem::Devices,
            NavItem::Firmware,
            NavItem::Settings,
        ];

        div()
            .w(px(200.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(BG_SIDEBAR))
            .border_r_1()
            .border_color(rgb(BORDER))
            .child(
                div()
                    .px_4()
                    .py_4()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_color(rgb(ACCENT)).text_xl().child("⚡"))
                    .child(
                        div()
                            .text_color(rgb(ACCENT))
                            .font_weight(FontWeight::BOLD)
                            .child("Spark NT"),
                    ),
            )
            .child(div().mx_3().h(px(1.0)).bg(rgb(BORDER)))
            .child(div().flex_1().py_2().children(nav_items.map(|item| {
                let is_active = item == active;
                let bg = if is_active { BG_BUTTON } else { BG_SIDEBAR };
                let fg = if is_active { ACCENT } else { TEXT_SECONDARY };
                div()
                    .id(SharedString::from(format!("nav-{}", item.label())))
                    .mx_2()
                    .my(px(1.0))
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .gap_2()
                    .bg(rgb(bg))
                    .text_color(rgb(fg))
                    .hover(|s: gpui::StyleRefinement| s.bg(rgb(BG_PANEL)))
                    .child(div().w(px(20.0)).text_center().child(item.icon_char()))
                    .child(item.label())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.navigate(item, cx);
                    }))
            })))
            .child(div().mx_3().h(px(1.0)).bg(rgb(BORDER)))
            .child(
                div()
                    .px_4()
                    .py_3()
                    .text_xs()
                    .text_color(rgb(TEXT_DIM))
                    .child("v0.1.0 · GPUI 0.2.2"),
            )
    }

    fn render_dashboard(&mut self, cx: &mut Context<Self>) -> Div {
        let count = self.click_count;
        let online_count = self.devices.iter().filter(|d| d.status == "Online").count();
        let total_count = self.devices.len();

        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Dashboard"),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(stat_card("Devices", &format!("{}", total_count), ACCENT))
                    .child(stat_card("Online", &format!("{}", online_count), GREEN))
                    .child(stat_card("Clicks", &format!("{}", count), ACCENT2))
                    .child(stat_card("Firmware", "4 ver", YELLOW)),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(
                        div()
                            .id("action-btn")
                            .px_6()
                            .py_2()
                            .bg(rgb(BG_BUTTON))
                            .hover(|s: gpui::StyleRefinement| s.bg(rgb(BG_BUTTON_HOVER)))
                            .active(|s: gpui::StyleRefinement| s.bg(rgb(BG_BUTTON_ACTIVE)))
                            .rounded_lg()
                            .cursor_pointer()
                            .text_color(rgb(ACCENT))
                            .font_weight(FontWeight::MEDIUM)
                            .child("Click to Test")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.increment(cx);
                            })),
                    )
                    .child(
                        div()
                            .id("scan-btn")
                            .px_6()
                            .py_2()
                            .bg(rgb(BG_CARD))
                            .hover(|s: gpui::StyleRefinement| s.bg(rgb(BG_BUTTON_HOVER)))
                            .rounded_lg()
                            .cursor_pointer()
                            .border_1()
                            .border_color(rgb(BORDER))
                            .text_color(rgb(TEXT_SECONDARY))
                            .child("Scan Devices")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.log_messages
                                    .push("[SCAN]  Scanning for devices...".into());
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT_SECONDARY))
                            .child("RECENT DEVICES"),
                    )
                    .children(self.devices.iter().take(3).map(|device| {
                        device_row(device)
                    })),
            )
    }

    fn render_devices(&self) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Devices"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(self.devices.iter().map(|device| device_row_full(device))),
            )
    }

    fn render_firmware(&self) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Firmware"),
            )
            .child(
                div()
                    .px_4()
                    .py_4()
                    .bg(rgb(BG_CARD))
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(BORDER))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_PRIMARY))
                            .child("Firmware management coming soon..."),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_DIM))
                            .child("Upload, flash and manage firmware for LILYGO devices."),
                    ),
            )
    }

    fn render_settings(&self) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(TEXT_PRIMARY))
                    .child("Settings"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(setting_row("Theme", "Dark (default)"))
                    .child(setting_row("Language", "English"))
                    .child(setting_row("Serial Port", "Auto-detect"))
                    .child(setting_row("Baud Rate", "115200"))
                    .child(setting_row("Flash Mode", "DIO")),
            )
    }

    fn render_status_bar(&self) -> Div {
        let online = self.devices.iter().filter(|d| d.status == "Online").count();
        let total = self.devices.len();
        div()
            .w_full()
            .h(px(28.0))
            .bg(rgb(BG_STATUS))
            .border_t_1()
            .border_color(rgb(BORDER))
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(div().size(px(6.0)).rounded_full().bg(rgb(GREEN)))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(TEXT_DIM))
                                    .child(format!("{}/{} online", online, total)),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(TEXT_DIM))
                            .child(format!("Clicks: {}", self.click_count)),
                    ),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(TEXT_DIM))
                    .child("GPUI 0.2.2 · Rust 2024"),
            )
    }

    fn render_log_panel(&self) -> Div {
        div()
            .w_full()
            .h(px(120.0))
            .bg(rgb(BG_SIDEBAR))
            .border_t_1()
            .border_color(rgb(BORDER))
            .flex()
            .flex_col()
            .child(
                div().px_3().py_1().child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(TEXT_DIM))
                        .child("OUTPUT"),
                ),
            )
            .child(
                div()
                    .flex_1()
                    .px_3()
                    .flex()
                    .flex_col()
                    .children(self.log_messages.iter().rev().take(8).map(|msg| {
                        let color = if msg.contains("[ERROR]") {
                            ACCENT2
                        } else if msg.contains("[NAV]") || msg.contains("[SCAN]") {
                            YELLOW
                        } else if msg.contains("[EVENT]") {
                            ACCENT
                        } else {
                            TEXT_DIM
                        };
                        div()
                            .text_xs()
                            .text_color(rgb(color))
                            .py(px(1.0))
                            .child(msg.clone())
                    })),
            )
    }
}

fn stat_card(title: &str, value: &str, color: u32) -> Div {
    div()
        .flex_1()
        .px_4()
        .py_3()
        .bg(rgb(BG_CARD))
        .rounded_lg()
        .border_1()
        .border_color(rgb(BORDER))
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(rgb(TEXT_DIM))
                .child(title.to_string()),
        )
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(color))
                .child(value.to_string()),
        )
}

fn device_row(device: &DeviceInfo) -> Div {
    let status_color = if device.status == "Online" {
        GREEN
    } else {
        ACCENT2
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .px_4()
        .py_3()
        .bg(rgb(BG_CARD))
        .rounded_lg()
        .border_1()
        .border_color(rgb(BORDER))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(div().size_2().rounded_full().bg(rgb(status_color)))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(TEXT_PRIMARY))
                                .child(device.name),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(TEXT_DIM))
                                .child(device.mcu),
                        ),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(status_color))
                .child(device.status),
        )
}

fn device_row_full(device: &DeviceInfo) -> Div {
    let status_color = if device.status == "Online" {
        GREEN
    } else {
        ACCENT2
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .px_4()
        .py_3()
        .bg(rgb(BG_CARD))
        .rounded_lg()
        .border_1()
        .border_color(rgb(BORDER))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(div().size(px(8.0)).rounded_full().bg(rgb(status_color)))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(TEXT_PRIMARY))
                                .child(device.name),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(TEXT_DIM))
                                .child(format!("{} · {}", device.mcu, device.firmware)),
                        ),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(status_color))
                .child(device.status),
        )
}

fn setting_row(label: &str, value: &str) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .px_4()
        .py_3()
        .bg(rgb(BG_CARD))
        .rounded_lg()
        .border_1()
        .border_color(rgb(BORDER))
        .child(
            div()
                .text_sm()
                .text_color(rgb(TEXT_PRIMARY))
                .child(label.to_string()),
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(TEXT_SECONDARY))
                .child(value.to_string()),
        )
}

impl Render for SparkApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BG_PRIMARY))
            .text_color(rgb(TEXT_PRIMARY))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .overflow_hidden()
                    .child(self.render_sidebar(cx))
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex_1()
                                    .p_6()
                                    .child(match self.active_nav {
                                        NavItem::Dashboard => {
                                            self.render_dashboard(cx).into_any_element()
                                        }
                                        NavItem::Devices => {
                                            self.render_devices().into_any_element()
                                        }
                                        NavItem::Firmware => {
                                            self.render_firmware().into_any_element()
                                        }
                                        NavItem::Settings => {
                                            self.render_settings().into_any_element()
                                        }
                                    }),
                            )
                            .child(self.render_log_panel()),
                    ),
            )
            .child(self.render_status_bar())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("LILYGO Spark NT".into()),
                ..Default::default()
            }),
            window_min_size: Some(Size {
                width: px(640.0),
                height: px(400.0),
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |_window, cx| cx.new(|_| SparkApp::new()))
            .unwrap();

        cx.activate(true);
    });
}
