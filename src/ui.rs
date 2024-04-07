use gpui::*;

#[derive(Debug)]
pub struct GameWindow {}

impl Render for GameWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(hsla(0.0, 0.0, 0.0, 1.0))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(hsla(1.0, 1.0, 1.0, 1.0))
            .child(format!("hello world"))
    }
}
