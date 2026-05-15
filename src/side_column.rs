//! Shared ~1/3-width left sidebar shell (search “Recent searches” + settings nav).

use gpui::{div, prelude::*, px, relative, Div};
use gpui::colors::Colors;

/// Left column frame: bordered panel sized to one-third of the row (matches search layout).
pub fn side_column_shell(colors: &Colors) -> Div {
    div()
        .flex_initial()
        .flex_basis(relative(1.0 / 3.0))
        .flex()
        .flex_col()
        .min_w(px(0.))
        .rounded_md()
        .border_1()
        .border_color(colors.border)
        .bg(colors.container)
        .p_4()
        .text_sm()
        .text_color(colors.text)
}

pub fn recent_searches_sidebar(colors: &Colors) -> Div {
    side_column_shell(colors)
        .child("Recent searches")
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .text_color(colors.disabled)
                .child("No recent searches"),
        )
}

/// Settings nav column: wider than `side_column_shell` (~⅓) so tab labels stay on one line.
pub fn settings_side_column_shell(colors: &Colors) -> Div {
    div()
        .flex_initial()
        .flex_basis(relative(0.4))
        .min_w(px(240.))
        .flex()
        .flex_col()
        .rounded_md()
        .border_1()
        .border_color(colors.border)
        .bg(colors.container)
        .p_4()
        .text_sm()
        .text_color(colors.text)
}
