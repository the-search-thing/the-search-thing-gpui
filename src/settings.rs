//! Settings area: tab identifiers and static tab bodies (see gpui-ce `examples/learn/creating_components.rs`).

use gpui::{div, prelude::*, Div, FontWeight, Stateful};
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

pub fn render_tab_content(tab: SettingsTab, colors: &Colors) -> impl IntoElement {
    match tab {
        SettingsTab::General => div()
            .flex()
            .flex_col()
            .gap_3()
            .text_sm()
            .text_color(colors.text)
            .child("General")
            .child(
                div()
                    .text_xs()
                    .text_color(colors.disabled)
                    .child("Application preferences will go here."),
            ),
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
            ),
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
            ),
    }
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
