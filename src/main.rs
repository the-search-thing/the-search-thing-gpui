mod input_dialog;

use gpui::{
    App, Application, Bounds, Context, Entity, Focusable, Render, TitlebarOptions, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};
use gpui::colors::Colors;

use input_dialog::{register_text_input_keybindings, TextInput};

struct LayoutExample {
    dialog_input: Entity<TextInput>,
}

impl Render for LayoutExample {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let colors = Colors::for_appearance(window);

        div()
            .id("main")
            .size_full()
            .flex()
            .flex_col()
            .w_full()
            .bg(colors.background)
            .child(self.dialog_input.clone())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .flex_1()
                    .w_full()
                    .min_h(px(0.))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .w_full()
                    .min_h(px(0.))
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_color(colors.text)
                            .text_center()
                            .child("welcome to the-search-thing"),
                    )
                    .child(
                        div()
                            .text_color(colors.disabled)
                            .text_center()
                            .child("Please start typing to get started..."),
                    )
                    // .child(
                        // div()
                        //     .flex_initial()
                        //     .flex_basis(relative(1.0 / 3.0))
                        //     .flex()
                        //     .flex_col()
                        //     .rounded_md()
                        //     .border_1()
                        //     .border_color(colors.border)
                        //     .bg(colors.container)
                        //     .p_4()
                        //     .text_sm()
                        //     .text_color(colors.text)
                        //     .child("Recent searches"),
                    // )
                    // .child(
                    //     div()
                    //         .flex_1()
                    //         .flex()
                    //         .flex_col()
                    //         .rounded_md()
                    //         .border_1()
                    //         .border_color(colors.border)
                    //         .bg(colors.container)
                    //         .p_4()
                    //         .text_sm()
                    //         .text_color(colors.text)
                    //         .child("Right column"),
                    // ),
            )
    }
}
fn main() {
    Application::new().run(|cx: &mut App| {
        register_text_input_keybindings(cx);

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
                    cx.new(|cx| LayoutExample {
                        dialog_input: cx.new(|cx| {
                            TextInput::new(cx, "Search for files, images, videos...").with_placeholder_color(
                                gpui::white().opacity(0.45),
                            )
                        }),
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
