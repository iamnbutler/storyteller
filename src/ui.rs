use gpui::*;

use crate::{input::Input, Game};

#[derive(Debug)]
pub struct GameWindow {
    pub game: Game,
    pub test_input: View<Input>,
}

impl GameWindow {
    pub fn new(cx: &mut ViewContext<Self>, game: Game) -> Self {
        let test_input = cx.new_view(|cx| Input::new(cx, "test-input", "test"));

        Self { game, test_input }
    }
}

impl Render for GameWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let player = self.game.data.read(cx).player.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(hsla(1.0, 1.0, 1.0, 1.0))
            .child(format!("Hello, {}!", player.name))
            .child(self.test_input.clone())
    }
}
