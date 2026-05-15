//! Settings area: tab identifiers and tab bodies (see gpui-ce `examples/learn/creating_components.rs`).
//! General tab layout mirrors `client/app/components/settings/General.tsx` (static UI only).

use gpui::{div, prelude::*, AnyElement, Div, FontWeight, Stateful, px};
use gpui::colors::Colors;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SettingsTab {
    General,
    Keybinds,
    About,
}

impl SettingsTab {
    pub const TABS: [SettingsTab; 3] = [Self::General, Self::Keybinds, Self::About];

    pub fn label(self) -> &'static str {
        match self {
            SettingsTab::General => "General",
            SettingsTab::Keybinds => "Keybinds",
            SettingsTab::About => "About",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            SettingsTab::General => "settings-tab-general",
            SettingsTab::Keybinds => "settings-tab-keybinds",
            SettingsTab::About => "settings-tab-about",
        }
    }
}

/// One vertical tab in the settings sidebar (left column).
pub fn settings_nav_item(colors: &Colors, tab: SettingsTab, active: bool) -> Stateful<Div> {
    let bg = if active {
        colors.container
    } else {
        colors.background
    };
    let border_clr = if active {
        colors.border
    } else {
        colors.background
    };

    div()
        .id(tab.id())
        .w_full()
        .whitespace_nowrap()
        .px_3()
        .py_2()
        .rounded_md()
        .cursor_pointer()
        .bg(bg)
        .text_xs()
        .font_weight(if active {
            FontWeight::SEMIBOLD
        } else {
            FontWeight::NORMAL
        })
        .text_color(if active {
            colors.text
        } else {
            colors.disabled
        })
        .border_1()
        .border_color(border_clr)
        .child(tab.label())
}

pub fn render_tab_content(tab: SettingsTab, colors: &Colors) -> AnyElement {
    match tab {
        SettingsTab::General => render_general_tab(colors).into_any_element(),
        SettingsTab::Keybinds => div()
            .flex()
            .flex_col()
            .gap_3()
            .text_sm()
            .text_color(colors.text)
            .child("Keybinds")
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .text_xs()
                    .children([
                        keybind_row(colors, "Open Settings", "cmd-, / ctrl-,"),
                        keybind_row(colors, "Leave Settings", "escape (when Settings is open)"),
                    ]),
            )
            .into_any_element(),
        SettingsTab::About => div()
            .flex()
            .flex_col()
            .gap_3()
            .text_sm()
            .text_color(colors.text)
            .child("About")
            .child(
                div()
                    .text_xs()
                    .text_color(colors.disabled)
                    .child("the-search-thing")
                    .child("by the-search-company"),
            )
            .into_any_element(),
    }
}

/// Static copy of the web `General.tsx` panel: card shell, header actions, rows (no wiring yet).
fn render_general_tab(colors: &Colors) -> impl IntoElement {
    let muted_action = || {
        div()
            .text_xs()
            .px_2()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(colors.border)
            .text_color(colors.disabled)
            .child("Discard")
    };
    let save_action = || {
        div()
            .text_xs()
            .px_2()
            .py_1()
            .rounded_md()
            .border_1()
            .border_color(colors.border)
            .text_color(colors.disabled)
            .child("Save")
    };

    div()
        .id("settings-general-panel")
        .flex()
        .flex_col()
        .gap_3()
        .w_full()
        .h_full()
        .min_h(px(0.))
        .min_w(px(0.))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(colors.disabled)
                                .child("GENERAL"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(muted_action())
                        .child(save_action()),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(general_settings_row(
                    colors,
                    row_heading(colors, "Launch at startup", "Open the app when you sign in."),
                    pill_button(colors, "Off"),
                ))
                .child(general_settings_row(
                    colors,
                    row_heading(colors, "Theme", "Choose light or dark mode."),
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(segment_button(colors, "Dark", true))
                        .child(segment_button(colors, "Light", false)),
                ))
                .child(general_settings_row(
                    colors,
                    row_heading(colors, "Font", "Choose Sans-Serif or Mono."),
                    select_like(colors, "Manrope"),
                ))
                // .child(general_settings_row(
                //     colors,
                //     row_heading(colors, "Search scope", "Files, folders, or both."),
                //     select_like(colors, "Everything"),
                // ))
                .child(general_settings_row(
                    colors,
                    row_heading(
                        colors,
                        "Window placement",
                        "Choose where the window appears.",
                    ),
                    select_like(colors, "Centered"),
                ))
                .child(general_settings_row(
                    colors,
                    row_heading(colors, "Clear recent searches", "Remove cached query history."),
                    pill_button(colors, "Clear"),
                ))
                .child(general_settings_row(
                    colors,
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .flex_1()
                        .min_w(px(0.))
                        .child(div().text_sm().text_color(colors.text).child("Clear Index"))
                        .child(
                            div()
                                .text_xs()
                                .text_color(colors.disabled)
                                .child(
                                    "Permanently removes all indexed files and embeddings. You will need to run a full re-index to search again.",
                                ),
                        ),
                    pill_button(colors, "Clear"),
                )),
        )
}

fn row_heading(colors: &Colors, title: &'static str, subtitle: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_0()
        .min_w(px(0.))
        .flex_1()
        .child(div().text_sm().text_color(colors.text).child(title))
        .child(
            div()
                .text_xs()
                .text_color(colors.disabled)
                .child(subtitle),
        )
}

fn general_settings_row(
    _colors: &Colors,
    label: impl IntoElement,
    control: impl IntoElement,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_start()
        .justify_between()
        .gap_3()
        .child(label)
        .child(
            div()
                .flex_initial()
                .flex_shrink_0()
                .child(control),
        )
}

fn pill_button(colors: &Colors, label: &'static str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .px_2()
        .py_1()
        .rounded_md()
        .text_xs()
        .text_color(colors.text)
        .bg(colors.background)
        .border_1()
        .border_color(colors.border)
        .child(label)
}

fn segment_button(colors: &Colors, label: &'static str, active: bool) -> impl IntoElement {
    let bg = if active {
        colors.container
    } else {
        colors.background
    };
    let border = if active {
        colors.text
    } else {
        colors.border
    };
    div()
        .flex()
        .items_center()
        .px_2()
        .py_1()
        .rounded_md()
        .text_xs()
        .text_color(colors.text)
        .bg(bg)
        .border_1()
        .border_color(border)
        .child(label)
}

fn select_like(colors: &Colors, value: &'static str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .px_2()
        .py_1()
        .rounded_md()
        .text_xs()
        .text_color(colors.text)
        .bg(colors.background)
        .border_1()
        .border_color(colors.border)
        .child(value)
}

fn keybind_row(colors: &Colors, label: &'static str, keys: &'static str) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .justify_between()
        .gap_4()
        .py_1()
        .border_b_1()
        .border_color(colors.border)
        .child(div().child(label))
        .child(
            div()
                .text_color(colors.disabled)
                .child(keys),
        )
}
