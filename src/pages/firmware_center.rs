use gpui::*;

use crate::app::SparkApp;
use crate::theme::*;

struct ProductInfo {
    name: &'static str,
    mcu: &'static str,
    description: &'static str,
}

const PRODUCTS: &[ProductInfo] = &[
    ProductInfo { name: "T-Display S3", mcu: "ESP32-S3", description: "1.9\" LCD, USB-C, Wi-Fi/BLE" },
    ProductInfo { name: "T-Display S3 AMOLED", mcu: "ESP32-S3", description: "1.91\" AMOLED, Touch, USB-C" },
    ProductInfo { name: "T-Deck", mcu: "ESP32-S3", description: "Keyboard, 2.8\" LCD, LoRa" },
    ProductInfo { name: "T-Watch S3", mcu: "ESP32-S3", description: "Wearable, Touch, IMU" },
    ProductInfo { name: "T-Beam Supreme", mcu: "ESP32-S3", description: "GPS, LoRa, Solar" },
    ProductInfo { name: "T-ETH-Lite", mcu: "ESP32-S3", description: "Ethernet, PoE, Wi-Fi" },
];

struct FirmwareInfo {
    name: &'static str,
    version: &'static str,
    fw_type: &'static str,
    size: &'static str,
}

const FIRMWARES: &[FirmwareInfo] = &[
    FirmwareInfo { name: "Factory Test", version: "v1.0.0", fw_type: "factory", size: "1.2 MB" },
    FirmwareInfo { name: "LVGL Demo", version: "v9.2.0", fw_type: "lvgl", size: "2.8 MB" },
    FirmwareInfo { name: "MicroPython", version: "v1.23.0", fw_type: "micropython", size: "1.5 MB" },
    FirmwareInfo { name: "Arduino Blink", version: "v1.0.0", fw_type: "bin", size: "256 KB" },
];

impl SparkApp {
    pub fn render_firmware_center(&self) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .flex_row()
            .overflow_hidden()
            // Left panel - product list
            .child(
                div()
                    .w(px(300.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .border_r_1()
                    .border_color(glass_border())
                    .child(
                        // Search bar
                        div()
                            .p_3()
                            .border_b_1()
                            .border_color(glass_border())
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .px_3()
                                    .py_2()
                                    .rounded_lg()
                                    .bg(hsla(0., 0., 0., 0.2))
                                    .border_1()
                                    .border_color(glass_border())
                                    .child(
                                        div().text_sm().text_color(rgb(TEXT_MUTED)).child("🔍 Search products..."),
                                    ),
                            ),
                    )
                    .child(
                        // Product list
                        {
                            let mut list = div()
                                .id("product-list")
                                .flex_1()
                                .overflow_y_scroll()
                                .p_2()
                                .flex()
                                .flex_col()
                                .gap_1();

                            for (i, product) in PRODUCTS.iter().enumerate() {
                                let is_selected = i == 0;
                                let mut item = div()
                                    .id(SharedString::from(format!("product-{}", i)))
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .px_3()
                                    .py(px(10.0))
                                    .rounded_xl()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));

                                if is_selected {
                                    item = item
                                        .bg(hsla(270. / 360., 0.4, 0.5, 0.10))
                                        .border_1()
                                        .border_color(hsla(270. / 360., 0.5, 0.5, 0.3))
                                        .shadow_lg();
                                }

                                // Product image placeholder
                                item = item.child(
                                    div()
                                        .w(px(48.0))
                                        .h(px(48.0))
                                        .rounded_lg()
                                        .bg(hsla(220. / 360., 0.1, 0.2, 0.5))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            div().text_color(rgb(TEXT_MUTED)).child("📱"),
                                        ),
                                );

                                item = item.child(
                                    div()
                                        .flex_1()
                                        .flex()
                                        .flex_col()
                                        .gap(px(2.0))
                                        .child(
                                            div().text_sm().text_color(rgb(TEXT_PRIMARY)).child(product.name.to_string()),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .px(px(6.0))
                                                        .py(px(1.0))
                                                        .rounded_sm()
                                                        .bg(hsla(270. / 360., 0.3, 0.3, 0.2))
                                                        .text_color(rgb(PRIMARY))
                                                        .child(product.mcu.to_string()),
                                                ),
                                        ),
                                );

                                list = list.child(item);
                            }

                            list
                        },
                    ),
            )
            // Right panel - firmware list
            .child(
                div()
                    .id("firmware-list")
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_6()
                    .gap_4()
                    .overflow_y_scroll()
                    .child(
                        // Product header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div().text_2xl().text_color(rgb(TEXT_PRIMARY)).child("T-Display S3"),
                                    )
                                    .child(
                                        div().text_sm().text_color(rgb(TEXT_MUTED)).child("1.9\" LCD, USB-C, Wi-Fi/BLE"),
                                    ),
                            )
                            .child(
                                div()
                                    .w(px(80.0))
                                    .h(px(80.0))
                                    .rounded_xl()
                                    .bg(hsla(220. / 360., 0.1, 0.2, 0.5))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div().text_2xl().child("📱"),
                                    ),
                            ),
                    )
                    .child(
                        // Firmware items
                        {
                            let mut items = div().flex().flex_col().gap_3();
                            for fw in FIRMWARES {
                                let (badge_color, badge_bg) = match fw.fw_type {
                                    "factory" => (GREEN, hsla(150. / 360., 0.6, 0.4, 0.15)),
                                    "micropython" => (AMBER, hsla(40. / 360., 0.7, 0.5, 0.15)),
                                    "lvgl" => (PRIMARY, hsla(270. / 360., 0.5, 0.5, 0.15)),
                                    _ => (TEXT_MUTED, hsla(0., 0., 0.3, 0.15)),
                                };

                                items = items.child(
                                    glass_card_div()
                                        .p_4()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .hover(|s| s.border_color(glass_border_hover()))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(div().text_color(rgb(TEXT_PRIMARY)).child(fw.name.to_string()))
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .px_2()
                                                                .py(px(2.0))
                                                                .rounded_md()
                                                                .bg(badge_bg)
                                                                .text_color(rgb(badge_color))
                                                                .child(fw.fw_type.to_string()),
                                                        ),
                                                )
                                                .child(
                                                    div().text_xs().text_color(rgb(TEXT_MUTED)).child(format!("{} · {}", fw.version, fw.size)),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .px_4()
                                                .py(px(6.0))
                                                .rounded_lg()
                                                .bg(hsla(220. / 360., 0.6, 0.5, 0.15))
                                                .text_sm()
                                                .text_color(rgb(0x3b82f6))
                                                .cursor_pointer()
                                                .hover(|s| s.bg(hsla(220. / 360., 0.6, 0.5, 0.25)))
                                                .child("⬇ Download"),
                                        ),
                                );
                            }
                            items
                        },
                    ),
            )
    }
}
