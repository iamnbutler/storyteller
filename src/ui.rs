use gpui::*;

use crate::{input::Input, Game};

pub enum GameWindowEvent {
    Focus,
    Blur,
}

impl EventEmitter<GameWindowEvent> for GameWindow {}

#[derive(Debug)]
pub struct GameWindow {
    focus_handle: FocusHandle,

    pub game: Game,
    pub test_input: View<Input>,
}

impl GameWindow {
    pub fn new(cx: &mut ViewContext<Self>, game: Game) -> Self {
        let focus_handle = cx.focus_handle();
        let test_input =
            cx.new_view(|cx| Input::new(cx, "test-input", "").set_placeholder("Type something..."));

        Self {
            focus_handle,
            game,
            test_input,
        }
    }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(GameWindowEvent::Focus);
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(GameWindowEvent::Blur);

        cx.notify();
    }
}

impl Render for GameWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let player = self.game.data.read(cx).player.clone();

        div()
            .id("game-window")
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .bg(hsla(1.0, 1.0, 1.0, 1.0))
            .text_xl()
            .text_color(hsla(0.0, 0.0, 0.0, 1.0))
            .on_click(cx.listener(|this, _event, cx| cx.focus_self()))
            .child(format!("Hello, {}!", player.name))
            .child(self.test_input.clone())
    }
}

impl FocusableView for GameWindow {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
