use gpui::*;
use gpui::prelude::FluentBuilder;

use crate::app::SparkApp;
// Firmware Center page - product list with collapsible series and firmware details
use crate::theme::*;

fn format_size(bytes: Option<u64>) -> String {
    match bytes {
        Some(b) if b >= 1024 * 1024 => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
        Some(b) if b >= 1024 => format!("{:.1} KB", b as f64 / 1024.0),
        Some(b) => format!("{} B", b),
        None => "—".to_string(),
    }
}

impl SparkApp {
    pub fn render_firmware_center(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let product_count = self.filtered_products().len();
        let selected_idx = self.selected_product_idx;

        // Get selected product info
        let selected_name = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.name.clone())
            .unwrap_or_else(|| self.i18n.t("fc.select_device").to_string());
        let selected_desc = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.description.clone())
            .unwrap_or_default();
        let selected_mcu = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.mcu.clone())
            .unwrap_or_default();
        let selected_github = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.github_repo.clone())
            .unwrap_or_default();
        let selected_product_page = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.product_page.clone())
            .unwrap_or_default();
        let selected_product_id = selected_idx
            .and_then(|i| self.flat_products.get(i))
            .map(|p| p.product_id.clone())
            .unwrap_or_default();
        let selected_image = self.product_images.get(&selected_product_id).cloned();
        let firmware_count = self.selected_firmwares.len();
        let has_selection = selected_idx.is_some();

        // Build firmware items
        let mut firmware_list = div().px_6().pb_6().flex().flex_col().gap_3();

        if firmware_count == 0 && has_selection {
            firmware_list = firmware_list.child(
                div()
                    .p_8()
                    .border_1()
                    .border_color(glass_border())
                    .rounded_xl()
                    .text_center()
                    .text_sm()
                    .text_color(rgb(TEXT_MUTED))
                    .child(self.i18n.t("fc.no_firmware").to_string()),
            );
        }

        for (fw_idx, fw) in self.selected_firmwares.iter().enumerate() {
            let (badge_color, badge_bg, badge_label) = match fw.fw_type.as_str() {
                "factory" => (GREEN, hsla(150. / 360., 0.6, 0.4, 0.15), "FACTORY"),
                "micropython" => (AMBER, hsla(40. / 360., 0.7, 0.5, 0.15), "MICROPYTHON"),
                "lvgl" => (0x8b5cf6, hsla(270. / 360., 0.5, 0.5, 0.15), "LVGL"),
                "lora" => (0xa855f7, hsla(280. / 360., 0.5, 0.5, 0.15), "LORA"),
                "bin" => (TEXT_SECONDARY, hsla(0., 0., 0.3, 0.15), "REPO"),
                _ => (0x3b82f6, hsla(220. / 360., 0.5, 0.5, 0.15), &*fw.fw_type),
            };
            let badge_label = badge_label.to_string();
            let size_str = format_size(fw.size);
            let fw_name = fw.name.clone();
            let fw_version = fw.version.clone();
            let fw_filename = fw.filename.clone();
            let fw_download_url = fw.download_url.clone();
            let fw_oss_url = fw.oss_url.clone();
            let fw_source_code_url = fw.source_code_url.clone();
            let fw_author_name = fw.author_name.clone();
            let fw_author_link = fw.author_link.clone();
            let fw_release_note = fw.release_note.clone();
            let fw_compressed_size = fw.compressed_size;
            let fw_size = fw.size;
            let primary = self.primary();

            // Preferred download URL (OSS if available, else original)
            let preferred_url = fw_oss_url.clone().unwrap_or_else(|| fw_download_url.clone());
            let fw_id = format!("fw-dl-{}-{}", fw_idx, fw.filename);

            firmware_list = firmware_list.child(
                glass_card_div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .hover(|s| s.border_color(hsla(0., 0., 0.5, 0.2)))
                    // Top row: name + badge + download button
                    .child(
                        div()
                            .flex()
                            .items_start()
                            .justify_between()
                            .gap_3()
                            .child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    // Name + type badge
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .flex_wrap()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_color(rgb(TEXT_PRIMARY))
                                                    .child(fw_name),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .px_2()
                                                    .py(px(2.0))
                                                    .rounded_full()
                                                    .bg(badge_bg)
                                                    .text_color(rgb(badge_color))
                                                    .child(badge_label),
                                            ),
                                    )
                                    // Metadata line: version, filename, size, compressed
                                    .child({
                                        let mut meta_parts: Vec<String> = Vec::new();
                                        meta_parts.push(format!("{}: {}", self.i18n.t("fc.version"), fw_version));
                                        if !fw_filename.is_empty() {
                                            meta_parts.push(format!("{}: {}", self.i18n.t("fc.file"), fw_filename));
                                        }
                                        meta_parts.push(size_str.clone());

                                        let mut meta_row = div()
                                            .flex()
                                            .items_center()
                                            .flex_wrap()
                                            .gap_x_3()
                                            .gap_y_1()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED));

                                        for part in &meta_parts {
                                            meta_row = meta_row.child(
                                                div().child(part.clone()),
                                            );
                                        }

                                        // Compressed size with compression ratio
                                        if let (Some(comp), Some(orig)) = (fw_compressed_size, fw_size) {
                                            if orig > 0 {
                                                let ratio = ((1.0 - comp as f64 / orig as f64) * 100.0) as i32;
                                                meta_row = meta_row.child(
                                                    div()
                                                        .text_color(rgb(GREEN))
                                                        .child(format!(
                                                            "ZIP {} ({}%)",
                                                            format_size(Some(comp)),
                                                            ratio
                                                        )),
                                                );
                                            }
                                        }

                                        meta_row
                                    })
                                    // Links row: OSS, Origin, Source Code, Author
                                    .child({
                                        let mut links = div()
                                            .flex()
                                            .items_center()
                                            .flex_wrap()
                                            .gap_x_3()
                                            .gap_y_1()
                                            .text_xs();

                                        // OSS link
                                        if let Some(oss) = fw_oss_url.clone() {
                                            let oss_click = oss.clone();
                                            links = links.child(
                                                div()
                                                    .id(SharedString::from(format!("oss-{}", fw_idx)))
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .text_color(rgb(0x3b82f6))
                                                    .cursor_pointer()
                                                    .hover(|s| s.text_color(rgb(0x60a5fa)))
                                                    .on_click(move |_, _, _| { let _ = open::that(&oss_click); })
                                                    .child("↗ OSS"),
                                            );
                                        }

                                        // Origin link
                                        if !fw_download_url.is_empty() {
                                            let origin_click = fw_download_url.clone();
                                            links = links.child(
                                                div()
                                                    .id(SharedString::from(format!("origin-{}", fw_idx)))
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .text_color(rgb(TEXT_MUTED))
                                                    .cursor_pointer()
                                                    .hover(|s| s.text_color(rgb(primary)))
                                                    .on_click(move |_, _, _| { let _ = open::that(&origin_click); })
                                                    .child(format!("↗ {}", self.i18n.t("fc.origin"))),
                                            );
                                        }

                                        // Source code link
                                        if let Some(src) = fw_source_code_url {
                                            let src_click = src.clone();
                                            links = links.child(
                                                div()
                                                    .id(SharedString::from(format!("src-{}", fw_idx)))
                                                    .flex()
                                                    .items_center()
                                                    .gap_1()
                                                    .text_color(rgb(TEXT_MUTED))
                                                    .cursor_pointer()
                                                    .hover(|s| s.text_color(rgb(primary)))
                                                    .on_click(move |_, _, _| { let _ = open::that(&src_click); })
                                                    .child(format!("🐙 {}", self.i18n.t("fc.source_code"))),
                                            );
                                        }

                                        // Author
                                        if let Some(author) = fw_author_name {
                                            if let Some(author_url) = fw_author_link {
                                                let author_click = author_url.clone();
                                                links = links.child(
                                                    div()
                                                        .id(SharedString::from(format!("author-{}", fw_idx)))
                                                        .flex()
                                                        .items_center()
                                                        .gap_1()
                                                        .text_color(rgb(AMBER))
                                                        .cursor_pointer()
                                                        .hover(|s| s.text_color(rgb(0xfbbf24)))
                                                        .on_click(move |_, _, _| { let _ = open::that(&author_click); })
                                                        .child(format!("👤 {}", author)),
                                                );
                                            } else {
                                                links = links.child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_1()
                                                        .text_color(rgb(AMBER))
                                                        .child(format!("👤 {}", author)),
                                                );
                                            }
                                        }

                                        links
                                    }),
                            )
                            // Download button
                            .child(
                                div()
                                    .id(SharedString::from(fw_id))
                                    .px_4()
                                    .py(px(8.0))
                                    .rounded_lg()
                                    .bg(self.primary_alpha(0.15))
                                    .text_sm()
                                    .text_color(rgb(primary))
                                    .cursor_pointer()
                                    .flex_none()
                                    .hover(|s| s.bg(hsla(220. / 360., 0.6, 0.5, 0.25)))
                                    .on_click(move |_, _, _| {
                                        if !preferred_url.is_empty() {
                                            let _ = open::that(&preferred_url);
                                        }
                                    })
                                    .child(format!("⬇ {}", self.i18n.t("fc.download"))),
                            ),
                    )
                    // Release note
                    .when(fw_release_note.is_some(), |d: Div| {
                        if let Some(note) = &fw_release_note {
                            d.child(
                                div()
                                    .pt_2()
                                    .border_t_1()
                                    .border_color(glass_border())
                                    .text_sm()
                                    .text_color(rgb(TEXT_SECONDARY))
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_color(rgb(TEXT_MUTED))
                                                    .child(format!("{}:", self.i18n.t("fc.note"))),
                                            )
                                            .child(note.clone()),
                                    ),
                            )
                        } else {
                            d
                        }
                    }),
            );
        }

        let search_placeholder = self.i18n.t("fc.search").to_string();
        let only_with_firmware_label = self.i18n.t("fc.only_with_firmware").to_string();
        let products_label = self.i18n.t("fc.products").to_string();
        let is_checked = self.only_with_firmware;
        let primary = self.primary();
        let primary_alpha_15 = self.primary_alpha(0.15);
        let available_fw_label = self.i18n.t("fc.available_firmware").to_string();
        let select_device_label = self.i18n.t("fc.select_device").to_string();

        // Build search bar (needs cx.listener before build_product_list borrows cx)
        let search_bar = {
            let search_text = self.search_query.clone();
            let display_text = if search_text.is_empty() {
                format!("🔍 {}", search_placeholder)
            } else {
                format!("🔍 {}", search_text)
            };
            let text_color = if search_text.is_empty() { TEXT_MUTED } else { TEXT_PRIMARY };

            div()
                .id("search-input")
                .flex()
                .items_center()
                .px_3()
                .py(px(8.0))
                .rounded_lg()
                .bg(hsla(0., 0., 0., 0.2))
                .border_1()
                .border_color(glass_border())
                .cursor_text()
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    let key = &event.keystroke.key;
                    if key == "backspace" {
                        this.search_query.pop();
                        cx.notify();
                    } else if key == "escape" {
                        this.search_query.clear();
                        cx.notify();
                    } else if let Some(ch) = &event.keystroke.key_char {
                        if !event.keystroke.modifiers.platform
                            && !event.keystroke.modifiers.control
                        {
                            this.search_query.push_str(ch);
                            cx.notify();
                        }
                    }
                }))
                .child(div().text_sm().text_color(rgb(text_color)).child(display_text))
                .when(!search_text.is_empty(), |d| {
                    d.child(div().flex_1()).child(
                        div()
                            .id("search-clear")
                            .text_xs()
                            .text_color(rgb(TEXT_MUTED))
                            .cursor_pointer()
                            .hover(|s| s.text_color(rgb(TEXT_PRIMARY)))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.search_query.clear();
                                cx.notify();
                            }))
                            .child("✕"),
                    )
                })
        };

        // Build filter toggle
        let filter_toggle = {
            let mut cb = div()
                .w(px(14.0))
                .h(px(14.0))
                .rounded_sm()
                .flex()
                .items_center()
                .justify_center();
            if is_checked {
                cb = cb.bg(rgb(primary)).child(div().text_xs().text_color(rgb(0xffffff)).child("✓"));
            } else {
                cb = cb.border_1().border_color(glass_border());
            }

            div()
                .id("firmware-filter-toggle")
                .flex()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .child(cb)
                .child(div().text_xs().text_color(rgb(TEXT_SECONDARY)).child(only_with_firmware_label))
                .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child(format!("({} {})", product_count, products_label)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.only_with_firmware = !this.only_with_firmware;
                    cx.notify();
                }))
        };

        // Build product list (borrows self + cx mutably, must be after other cx.listener calls)
        let product_list_div = self.build_product_list(cx);

        // Build right panel content
        let right_panel = if has_selection {
            div()
                .id("firmware-detail-panel")
                .flex_1()
                .flex()
                .flex_col()
                .overflow_y_scroll()
                // Product header
                .child(
                    div()
                        .p_6()
                        .border_b_1()
                        .border_color(glass_border())
                        .bg(linear_gradient(
                            90.,
                            linear_color_stop(hsla(240. / 360., 0.15, 0.12, 1.0), 0.),
                            linear_color_stop(hsla(260. / 360., 0.12, 0.10, 1.0), 1.0),
                        ))
                        .child(
                            div()
                                .flex()
                                .items_start()
                                .justify_between()
                                .gap_4()
                                // Product image in header
                                .when(selected_image.is_some(), {
                                    let img_clone = selected_image.clone();
                                    move |d: Div| {
                                        d.child(
                                            div()
                                                .w(px(80.0))
                                                .h(px(80.0))
                                                .rounded_xl()
                                                .bg(rgb(0xffffff))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .flex_none()
                                                .shadow_md()
                                                .overflow_hidden()
                                                .child(
                                                    img(ImageSource::Render(img_clone.unwrap()))
                                                        .w(px(80.0))
                                                        .h(px(80.0))
                                                        .object_fit(ObjectFit::Cover),
                                                ),
                                        )
                                    }
                                })
                                .child(
                                    div()
                                        .flex_1()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child(div().text_2xl().text_color(rgb(TEXT_PRIMARY)).child(selected_name))
                                        .child(div().text_sm().text_color(rgb(TEXT_MUTED)).child(selected_desc))
                                        .when(!selected_mcu.is_empty(), |d: Div| {
                                            d.child(
                                                div()
                                                    .text_xs()
                                                    .px(px(8.0))
                                                    .py(px(2.0))
                                                    .rounded_md()
                                                    .bg(primary_alpha_15)
                                                    .text_color(rgb(primary))
                                                    .child(selected_mcu.clone())
                                            )
                                        })
                                        .child({
                                            let gh = selected_github.clone();
                                            let pp = selected_product_page.clone();
                                            div()
                                                .flex()
                                                .gap_2()
                                                .mt_2()
                                                .when(!gh.is_empty(), |d: Div| {
                                                    let gh = gh.clone();
                                                    d.child(
                                                        Self::header_action_btn("🐙", "GitHub Repo")
                                                            .id("gh-btn")
                                                            .cursor_pointer()
                                                            .on_click(move |_, _, _| { let _ = open::that(&gh); }),
                                                    )
                                                })
                                                .when(!pp.is_empty(), |d: Div| {
                                                    let pp = pp.clone();
                                                    d.child(
                                                        Self::header_action_btn("🌐", "Product Page")
                                                            .id("pp-btn")
                                                            .cursor_pointer()
                                                            .on_click(move |_, _, _| { let _ = open::that(&pp); }),
                                                    )
                                                })
                                        }),
                                )
                                .child(
                                    div()
                                        .w(px(96.0))
                                        .h(px(96.0))
                                        .rounded_xl()
                                        .bg(rgb(0xffffff))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .shadow_xl()
                                        .flex_none()
                                        .child(div().text_2xl().child("📱")),
                                ),
                        ),
                )
                // Available Firmware title
                .child(
                    div()
                        .px_6()
                        .pt_4()
                        .pb_2()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(div().text_color(rgb(primary)).child("📋"))
                                .child(div().text_lg().text_color(rgb(TEXT_PRIMARY)).child(available_fw_label))
                                .child(
                                    div()
                                        .text_xs()
                                        .px(px(8.0))
                                        .py(px(2.0))
                                        .rounded_full()
                                        .bg(primary_alpha_15)
                                        .text_color(rgb(primary))
                                        .child(format!("{}", firmware_count)),
                                ),
                        ),
                )
                .child(firmware_list)
                .into_any_element()
        } else {
            div()
                .id("firmware-detail-panel")
                .flex_1()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .overflow_y_scroll()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .text_color(rgb(TEXT_MUTED))
                        .child(div().text_2xl().pb_4().child("🔄"))
                        .child(select_device_label),
                )
                .into_any_element()
        };

        // Assemble layout
        div()
            .flex_1()
            .flex()
            .flex_row()
            .overflow_hidden()
            // Left panel
            .child(
                div()
                    .w(px(340.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .border_r_1()
                    .border_color(glass_border())
                    .bg(hsla(220. / 360., 0.1, 0.08, 0.5))
                    .child(
                        div()
                            .p(px(16.0))
                            .border_b_1()
                            .border_color(glass_border())
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(search_bar)
                            .child(filter_toggle),
                    )
                    .child(product_list_div),
            )
            // Right panel
            .child(right_panel)
    }

    /// Build the product list panel with collapsible series groups
    fn build_product_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut images_to_load: Vec<(String, String)> = Vec::new();
        let mut list = div()
            .id("product-list")
            .flex_1()
            .overflow_y_scroll()
            .p_2()
            .flex()
            .flex_col()
            .gap_1();

        if self.manifest_loading {
            // Loading skeleton
            for i in 0..6 {
                list = list.child(
                    div()
                        .id(SharedString::from(format!("skeleton-{}", i)))
                        .flex()
                        .items_center()
                        .gap_3()
                        .px_3()
                        .py(px(10.0))
                        .rounded_xl()
                        .child(
                            div()
                                .w(px(48.0))
                                .h(px(48.0))
                                .rounded_lg()
                                .bg(hsla(0., 0., 0., 0.15)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(div().h(px(14.0)).w(px(120.0)).rounded_sm().bg(hsla(0., 0., 0., 0.15)))
                                .child(div().h(px(10.0)).w(px(80.0)).rounded_sm().bg(hsla(0., 0., 0., 0.1))),
                        ),
                );
            }
            return list;
        }

        if let Some(err) = &self.manifest_error {
            return list.child(
                div()
                    .p_4()
                    .text_center()
                    .text_sm()
                    .text_color(rgb(RED))
                    .child(err.clone()),
            );
        }

        // Build series groups and standalone products
        let filtered = self.filtered_products();
        let filtered_ids: std::collections::HashSet<String> = filtered.iter().map(|(_, p)| p.product_id.clone()).collect();
        let selected_idx = self.selected_product_idx;

        for group in &self.manifest.product_list {
            let is_series = group.products.as_ref().is_some_and(|p| !p.is_empty());

            if is_series {
                let series_id = group.id.clone().unwrap_or_else(|| group.name.clone());
                let is_expanded = self.expanded_series.contains(&series_id);

                // Filter products in this series
                let products: Vec<_> = group.products.as_ref().unwrap()
                    .iter()
                    .filter(|p| filtered_ids.contains(&p.product_id))
                    .collect();

                if products.is_empty() {
                    continue;
                }

                let product_count = products.len();
                let series_id_for_click = series_id.clone();

                // Series header (collapsible)
                list = list.child(
                    div()
                        .id(SharedString::from(format!("series-{}", series_id)))
                        .flex()
                        .items_center()
                        .px_3()
                        .py(px(10.0))
                        .rounded_xl()
                        .cursor_pointer()
                        .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            if this.expanded_series.contains(&series_id_for_click) {
                                this.expanded_series.remove(&series_id_for_click);
                            } else {
                                this.expanded_series.insert(series_id_for_click.clone());
                            }
                            cx.notify();
                        }))
                        // Chevron
                        .child(
                            div()
                                .w(px(18.0))
                                .h(px(48.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .mr(px(4.0))
                                .text_color(rgb(TEXT_MUTED))
                                .child(if is_expanded { "▼" } else { "▶" }),
                        )
                        // Series image placeholder
                        .child(
                            div()
                                .w(px(48.0))
                                .h(px(48.0))
                                .rounded_lg()
                                .bg(rgb(0xffffff))
                                .flex()
                                .items_center()
                                .justify_center()
                                .flex_none()
                                .shadow_sm()
                                .mr_3()
                                .child(div().text_color(rgb(TEXT_MUTED)).child("📦")),
                        )
                        // Series info
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(rgb(TEXT_PRIMARY))
                                        .child(group.name.clone()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_xs()
                                                .px(px(6.0))
                                                .py(px(1.0))
                                                .rounded_sm()
                                                .bg(hsla(0., 0., 0., 0.15))
                                                .text_color(rgb(TEXT_SECONDARY))
                                                .child(format!("{} {}", product_count, self.i18n.t("fc.products"))),
                                        ),
                                )
                                .when(!group.description.is_empty(), |d: Div| {
                                    d.child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(TEXT_MUTED))
                                            .overflow_hidden()
                                            .child(group.description.clone()),
                                    )
                                }),
                        ),
                );

                // Expanded product items
                if is_expanded {
                    for product in products {
                        let real_idx = self.flat_products.iter().position(|p| p.product_id == product.product_id);
                        if let Some(idx) = real_idx {
                            let is_selected = selected_idx == Some(idx);
                            let primary = self.primary();

                            let mut item = div()
                                .id(SharedString::from(format!("product-{}", idx)))
                                .ml(px(22.0))
                                .pl(px(22.0))
                                .border_l_2()
                                .border_color(hsla(0., 0., 0.5, 0.1))
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_2()
                                .py(px(6.0))
                                .rounded_lg()
                                .cursor_pointer()
                                .hover(|s| s.bg(hsla(0., 0., 0.5, 0.05)));

                            if is_selected {
                                item = item
                                    .bg(self.primary_alpha(0.10))
                                    .border_color(self.primary_alpha(0.3));
                            }

                            // Product image (async loaded or placeholder)
                            let pid = product.product_id.clone();
                            let img_url = product.image_url.clone();
                            let image_container = div()
                                .w(px(40.0))
                                .h(px(40.0))
                                .rounded_md()
                                .bg(rgb(0xffffff))
                                .flex()
                                .items_center()
                                .justify_center()
                                .flex_none()
                                .shadow_sm()
                                .overflow_hidden();
                            if let Some(render_img) = self.product_images.get(&pid) {
                                item = item.child(
                                    image_container.child(
                                        img(ImageSource::Render(render_img.clone()))
                                            .w(px(40.0))
                                            .h(px(40.0))
                                            .object_fit(ObjectFit::Cover),
                                    ),
                                );
                            } else {
                                // Queue image for loading
                                if !img_url.is_empty()
                                    && !self.product_images.contains_key(&pid)
                                    && !self.loading_images.contains(&pid)
                                {
                                    images_to_load.push((pid, img_url));
                                }
                                item = item.child(
                                    image_container
                                        .child(div().text_xs().text_color(rgb(TEXT_MUTED)).child("📱")),
                                );
                            }

                            let name = product.name.clone();
                            let mcu = product.mcu.clone();

                            item = item.child(
                                div()
                                    .flex_1()
                                    .flex()
                                    .flex_col()
                                    .gap(px(1.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(if is_selected { rgb(primary) } else { rgb(TEXT_PRIMARY) })
                                            .child(name),
                                    )
                                    .when(!mcu.is_empty(), |d: Div| {
                                        d.child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(TEXT_MUTED))
                                                .child(mcu),
                                        )
                                    }),
                            );

                            item = item.on_click(cx.listener(move |this, _, _, cx| {
                                this.select_product(idx);
                                cx.notify();
                            }));

                            list = list.child(item);
                        }
                    }
                }
            } else if let Some(pid) = &group.product_id {
                // Standalone product (not in a series)
                if !filtered_ids.contains(pid) {
                    continue;
                }

                let real_idx = self.flat_products.iter().position(|p| &p.product_id == pid);
                if let Some(idx) = real_idx {
                    let is_selected = selected_idx == Some(idx);
                    let primary = self.primary();

                    let mut item = div()
                        .id(SharedString::from(format!("product-{}", idx)))
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
                            .bg(self.primary_alpha(0.10))
                            .border_1()
                            .border_color(self.primary_alpha(0.3))
                            .shadow_lg();
                    }

                    // Spacer to align with series items
                    item = item.child(div().w(px(18.0)).mr(px(4.0)));

                    // Product image placeholder
                    item = item.child(
                        div()
                            .w(px(48.0))
                            .h(px(48.0))
                            .rounded_lg()
                            .bg(rgb(0xffffff))
                            .flex()
                            .items_center()
                            .justify_center()
                            .flex_none()
                            .shadow_sm()
                            .child(div().text_color(rgb(TEXT_MUTED)).child("📱")),
                    );

                    let name = group.name.clone();
                    let mcu = group.mcu.clone().unwrap_or_default();
                    let desc = group.description.clone();

                    item = item.child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(if is_selected { rgb(primary) } else { rgb(TEXT_PRIMARY) })
                                    .child(name),
                            )
                            .when(!mcu.is_empty(), |d: Div| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_xs()
                                                .px(px(6.0))
                                                .py(px(1.0))
                                                .rounded_sm()
                                                .bg(self.primary_alpha(0.15))
                                                .text_color(rgb(self.primary()))
                                                .child(mcu),
                                        ),
                                )
                            })
                            .when(!desc.is_empty(), |d: Div| {
                                d.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(TEXT_MUTED))
                                        .overflow_hidden()
                                        .child(desc),
                                )
                            }),
                    );

                    item = item.on_click(cx.listener(move |this, _, _, cx| {
                        this.select_product(idx);
                        cx.notify();
                    }));

                    list = list.child(item);
                }
            }
        }

        // Trigger async image loading for visible products
        for (pid, url) in images_to_load {
            self.load_product_image(pid, url, cx);
        }

        list
    }

    fn header_action_btn(icon: &str, label: &str) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .px_2()
            .py(px(4.0))
            .rounded_lg()
            .bg(hsla(0., 0., 0., 0.15))
            .border_1()
            .border_color(glass_border())
            .text_xs()
            .text_color(rgb(TEXT_SECONDARY))
            .cursor_pointer()
            .hover(|s| s.bg(hsla(0., 0., 0., 0.25)).text_color(rgb(TEXT_PRIMARY)))
            .child(icon.to_string())
            .child(label.to_string())
    }
}
