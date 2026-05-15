mod input_dialog;
mod native_sidecar;
mod settings;
mod side_column;

use gpui::{
    App, Application, Bounds, ClickEvent, Context, Entity, FocusHandle, Focusable, FontWeight,
    KeyBinding, Render, Subscription, TitlebarOptions, Window, WindowBounds, WindowOptions,
    actions, div, prelude::*, px, size,
};
use gpui::colors::Colors;
use the_search_thing::sidecar::native_ipc::NativeSearchRow;

use input_dialog::{register_text_input_keybindings, SearchSubmitted, TextInput};
use settings::{render_tab_content, settings_nav_item, SettingsTab};
use side_column::{recent_searches_sidebar, settings_side_column_shell};

actions!(
    app_shell,
    [
        OpenSettings,
        CloseSettings,
    ]
);

fn register_app_keybindings(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-,", OpenSettings, None),
        KeyBinding::new("ctrl-,", OpenSettings, None),
        KeyBinding::new("escape", CloseSettings, None),
    ]);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppPage {
    Search,
    Settings,
}

struct AppShell {
    shell_focus: FocusHandle,
    dialog_input: Entity<TextInput>,
    /// [`cx.subscribe`] returns a [`Subscription`] that **must stay alive**; dropping it unsubscribes.
    _search_events: Subscription,
    search_busy: bool,
    search_error: Option<String>,
    search_results: Vec<NativeSearchRow>,
    page: AppPage,
    settings_tab: SettingsTab,
}

impl AppShell {
    fn on_search_submitted(&mut self, event: &SearchSubmitted, cx: &mut Context<Self>) {
        let q = event.query.trim().to_string();
        if q.is_empty() {
            return;
        }

        self.search_busy = true;
        self.search_error = None;
        self.search_results.clear();
        cx.notify();

        cx.spawn(async move |this, cx| {
            let outcome = cx
                .background_spawn(async move { native_sidecar::native_search(&q) })
                .await;

            let _ = this.update(cx, |layout, cx| {
                layout.search_busy = false;
                match outcome {
                    Ok(rows) => layout.search_results = rows,
                    Err(e) => layout.search_error = Some(e),
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn open_settings(&mut self, _: &OpenSettings, window: &mut Window, cx: &mut Context<Self>) {
        self.page = AppPage::Settings;
        window.focus(&self.shell_focus);
        cx.notify();
    }

    fn close_settings(&mut self, _: &CloseSettings, window: &mut Window, cx: &mut Context<Self>) {
        if self.page == AppPage::Settings {
            self.page = AppPage::Search;
            cx.notify();
            window.focus(&self.dialog_input.focus_handle(cx));
        }
    }
}

impl Render for AppShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Colors::for_appearance(window);
        let has_text = cx.read_entity(&self.dialog_input, |input, _app| !input.content.is_empty());
        let show_workspace = has_text
            || self.search_busy
            || self.search_error.is_some()
            || !self.search_results.is_empty();

        let settings_body = || {
            div()
                .flex()
                .flex_col()
                .size_full()
                .min_h(px(0.))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_1()
                        .w_full()
                        .min_h(px(0.))
                        .gap_4()
                        .child(
                            settings_side_column_shell(&colors).child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(colors.disabled)
                                            .child("Settings"),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .children(SettingsTab::TABS.iter().map(|&tab| {
                                                let active = self.settings_tab == tab;
                                                settings_nav_item(&colors, tab, active).on_click(
                                                    cx.listener(move |this, _: &ClickEvent, _, cx| {
                                                        this.settings_tab = tab;
                                                        cx.notify();
                                                    }),
                                                )
                                            })),
                                    ),
                            ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .p_4()
                                .min_h(px(0.))
                                .child(render_tab_content(self.settings_tab, &colors)),
                        ),
                )
        };

        let search_body = || {
            div()
                .flex()
                .flex_col()
                .flex_1()
                .size_full()
                .min_h(px(0.))
                .child(self.dialog_input.clone())
                .child(if show_workspace {
                    div()
                        .flex()
                        .flex_row()
                        .flex_1()
                        .w_full()
                        .min_h(px(0.))
                        .gap_4()
                        .child(recent_searches_sidebar(&colors))
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .rounded_md()
                                .border_1()
                                .border_color(colors.border)
                                .bg(colors.container)
                                .p_4()
                                .text_sm()
                                .text_color(colors.text)
                                .gap_2()
                                .child("Results (native IPC)")
                                .child(div().text_xs().text_color(colors.disabled).child(
                                    "Press Enter to search via framed bincode sidecar (HELIX must be up).",
                                ))
                                .child(if self.search_busy {
                                    div().child("Searching…")
                                } else if let Some(err) = self.search_error.as_ref() {
                                    div().text_color(gpui::red()).child(err.clone())
                                } else if self.search_results.is_empty() {
                                    div()
                                        .text_color(colors.disabled)
                                        .child("No hits yet — Enter runs search.query over native IPC.")
                                } else {
                                    div().flex().flex_col().gap_2().children(
                                        self.search_results.iter().map(|row| {
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                                        .child(row.path.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_color(colors.disabled)
                                                        .child(format!("kind: {}", row.label)),
                                                )
                                        }),
                                    )
                                }
                                ),
                        )
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .w_full()
                        .min_h(px(0.))
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .rounded_md()
                        .border_1()
                        .border_color(colors.border)
                        .bg(colors.container)
                        .p_4()
                        .child(
                            div()
                                .text_color(colors.text)
                                .text_center()
                                .child("the-search-thing"),
                        )
                        .child(
                            div()
                                .text_color(colors.disabled)
                                .text_center()
                                .child(
                                    "Start typing, then press Enter to search (native IPC sidecar).",
                                ),
                        )
                })
        };

        div()
            .id("app-shell-root")
            .size_full()
            .flex()
            .flex_col()
            .bg(colors.background)
            .track_focus(&self.shell_focus)
            .on_action(cx.listener(Self::open_settings))
            .on_action(cx.listener(Self::close_settings))
            .child(if self.page == AppPage::Settings {
                settings_body()
            } else {
                search_body()
            })
    }
}

impl Focusable for AppShell {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.shell_focus.clone()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        register_text_input_keybindings(cx);
        register_app_keybindings(cx);

        let bounds = Bounds::centered(None, size(px(800.), px(450.)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("the-search-thing".into()),
                        appears_transparent: false,
                        traffic_light_position: None,
                    }),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|cx| {
                        let dialog_input = cx.new(|cx| {
                            TextInput::new(cx, "Search for files, images, videos...")
                                .with_placeholder_color(gpui::white().opacity(0.45))
                        });

                        let search_events = cx.subscribe(
                            &dialog_input,
                            |shell: &mut AppShell, _input, event: &SearchSubmitted, cx| {
                                shell.on_search_submitted(event, cx);
                            },
                        );

                        AppShell {
                            shell_focus: cx.focus_handle(),
                            dialog_input,
                            _search_events: search_events,
                            search_busy: false,
                            search_error: None,
                            search_results: Vec::new(),
                            page: AppPage::Search,
                            settings_tab: SettingsTab::General,
                        }
                    })
                },
            )
            .expect("Failed to open window");

        window
            .update(cx, |view, window, cx| {
                window.focus(&view.dialog_input.focus_handle(cx));
                cx.activate(true);
            })
            .expect("focus dialog field");
    });
}
