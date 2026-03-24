use gpui::*;

use crate::app::{NewsItem, SparkApp};
use crate::theme::*;

impl SparkApp {
    pub fn render_discovery(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.i18n.t("discovery.title").to_string();
        let subtitle = self.i18n.t("discovery.subtitle").to_string();
        let primary = self.primary();

        // Auto-load news on first visit
        if self.news_items.is_empty() && !self.news_loading && self.news_error.is_none() {
            self.load_news(cx);
        }

        let mut page = div()
            .id("discovery-page")
            .flex_1()
            .flex()
            .flex_col()
            .overflow_hidden()
            // Header
            .child(
                div()
                    .p_6()
                    .border_b_1()
                    .border_color(glass_border())
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .w(px(40.))
                                            .h(px(40.))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded_xl()
                                            .bg(linear_gradient(
                                                135.,
                                                linear_color_stop(rgb(primary), 0.),
                                                linear_color_stop(rgb(0x7c3aed), 1.),
                                            ))
                                            .shadow(vec![BoxShadow {
                                                color: hsla(270. / 360., 0.7, 0.5, 0.3),
                                                offset: point(px(0.), px(2.)),
                                                blur_radius: px(8.),
                                                spread_radius: px(0.),
                                            }])
                                            .child(
                                                div().text_color(rgb(0xffffff)).child("📰"),
                                            ),
                                    )
                                    .child(
                                        div().text_2xl().text_color(rgb(TEXT_PRIMARY)).child(title),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(TEXT_MUTED))
                                    .child(subtitle),
                            ),
                    )
                    // Refresh button
                    .child(
                        div()
                            .id("refresh-news-btn")
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .cursor_pointer()
                            .text_color(rgb(TEXT_MUTED))
                            .hover(|s| s.bg(hsla(0., 0., 0.5, 0.1)).text_color(rgb(TEXT_PRIMARY)))
                            .child("🔄 Refresh")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.news_items.clear();
                                this.news_error = None;
                                this.load_news(cx);
                            })),
                    ),
            );

        // Content
        if self.news_loading {
            // Loading skeletons - 3 columns, 2 rows
            let mut rows = div().flex().flex_col().gap_6();
            for row in 0..2 {
                let mut row_div = div().flex().gap_6();
                for col in 0..3 {
                    row_div = row_div.child(Self::skeleton_card(row * 3 + col));
                }
                rows = rows.child(row_div);
            }
            page = page.child(
                div()
                    .id("discovery-loading")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(rows),
            );
        } else if let Some(err) = &self.news_error {
            // Error state with retry
            let err_msg = err.clone();
            page = page.child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_3()
                            .child(div().text_2xl().child("⚠️"))
                            .child(div().text_color(rgb(0xef4444)).child(err_msg))
                            .child(
                                div()
                                    .id("retry-news")
                                    .px_4()
                                    .py_2()
                                    .rounded_lg()
                                    .bg(rgb(primary))
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .child("Retry")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.news_error = None;
                                        this.load_news(cx);
                                    })),
                            ),
                    ),
            );
        } else {
            // News items in 3-column rows
            let items = &self.news_items;
            let mut rows = div().flex().flex_col().gap_6();
            let chunks: Vec<&[NewsItem]> = items.chunks(3).collect();
            let mut idx = 0;
            for chunk in chunks {
                let mut row_div = div().flex().gap_6();
                for item in chunk {
                    row_div = row_div.child(Self::dynamic_news_card(idx, item));
                    idx += 1;
                }
                // If last row has < 3 items, add spacers to keep card widths consistent
                for _ in chunk.len()..3 {
                    row_div = row_div.child(div().flex_1().min_w_0());
                }
                rows = rows.child(row_div);
            }
            page = page.child(
                div()
                    .id("discovery-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_6()
                    .child(rows),
            );
        }

        page
    }

    fn source_badge_color(source: &str) -> u32 {
        match source {
            "Hackaday" => 0x1a1a2e,
            "CNX Software" => 0x38bdf8,
            "Adafruit" => 0xbe185d,
            "Reddit" => 0xea580c,
            "GitHub" => 0x374151,
            _ => 0x6366f1,
        }
    }

    fn dynamic_news_card(idx: usize, item: &NewsItem) -> Stateful<Div> {
        let source_color = Self::source_badge_color(&item.source);
        let url = item.url.clone();
        let tags: Vec<String> = item.tags.clone();

        let mut card = glass_card_div()
            .id(SharedString::from(format!("news-{}", idx)))
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .overflow_hidden()
            .cursor_pointer()
            .hover(|s| s.border_color(glass_border_hover()).shadow_xl())
            .on_click(move |_, _, cx| {
                let _ = open::that(&url);
                cx.stop_propagation();
            })
            // Image placeholder area
            .child(
                div()
                    .h(px(192.0))
                    .w_full()
                    .bg(hsla(220. / 360., 0.08, 0.06, 0.9))
                    .flex()
                    .items_center()
                    .justify_center()
                    .relative()
                    .child(
                        div()
                            .text_size(px(48.0))
                            .text_color(hsla(0., 0., 0.25, 0.5))
                            .child("📰"),
                    )
                    // Source badge (absolute positioned)
                    .child(
                        div()
                            .absolute()
                            .top_3()
                            .left_3()
                            .px_2()
                            .py(px(4.0))
                            .rounded_lg()
                            .bg(rgb(source_color))
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(item.source.clone()),
                    ),
            )
            // Content area
            .child(
                div()
                    .p(px(20.0))
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    // Title
                    .child(
                        div()
                            .text_color(rgb(TEXT_PRIMARY))
                            .font_weight(FontWeight::BOLD)
                            .line_height(px(24.0))
                            .child(item.title.clone()),
                    )
                    // Summary
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(TEXT_MUTED))
                            .line_height(px(20.0))
                            .child(item.summary.clone()),
                    ),
            );

        // Footer with date, tags, external link icon
        let mut footer = div()
            .px(px(20.0))
            .pb(px(16.0))
            .pt(px(12.0))
            .border_t_1()
            .border_color(glass_border())
            .flex()
            .items_center()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .text_xs()
                    .text_color(rgb(TEXT_MUTED))
                    .child("📅")
                    .child(item.date.clone()),
            );

        for tag in &tags {
            footer = footer.child(
                div()
                    .text_xs()
                    .px(px(6.0))
                    .py(px(2.0))
                    .rounded_sm()
                    .bg(hsla(0., 0., 0., 0.15))
                    .text_color(rgb(TEXT_MUTED))
                    .child(format!("#{}", tag)),
            );
        }

        footer = footer.child(div().flex_1()).child(
            div()
                .w(px(28.0))
                .h(px(28.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_full()
                .bg(hsla(0., 0., 0.3, 0.1))
                .text_xs()
                .text_color(rgb(TEXT_MUTED))
                .child("↗"),
        );

        card = card.child(footer);
        card
    }

    fn skeleton_card(_idx: usize) -> Div {
        glass_card_div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                div()
                    .h(px(192.0))
                    .w_full()
                    .bg(hsla(220. / 360., 0.1, 0.08, 0.6)),
            )
            .child(
                div()
                    .p(px(20.0))
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .h(px(18.0))
                            .w(px(200.0))
                            .rounded_md()
                            .bg(hsla(0., 0., 0.3, 0.2)),
                    )
                    .child(
                        div()
                            .h(px(14.0))
                            .w(px(260.0))
                            .rounded_md()
                            .bg(hsla(0., 0., 0.3, 0.15)),
                    )
                    .child(
                        div()
                            .h(px(14.0))
                            .w(px(160.0))
                            .rounded_md()
                            .bg(hsla(0., 0., 0.3, 0.1)),
                    ),
            )
    }
}
