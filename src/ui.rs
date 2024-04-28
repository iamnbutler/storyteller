use gpui::*;

use crate::Game;

#[derive(Debug)]
pub struct GameWindow {
    pub game: Game,
}

impl GameWindow {
    pub fn new(game: Game) -> Self {
        Self { game }
    }
}

impl Render for GameWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let player = self.game.data.read(cx).player.clone();

        div()
            .flex()
            .bg(hsla(0.0, 0.0, 0.0, 1.0))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(hsla(1.0, 1.0, 1.0, 1.0))
            .child(format!("Hello, {}!", player.name))
    }
}
