use gpui::{prelude::*, *};

pub struct Cursor {
    pub visible: bool,
    pub position: Point<Pixels>,
}

impl Cursor {
    pub fn new(_cx: &mut ViewContext<Self>) -> Self {
        Self {
            visible: false,
            position: point(0.0.into(), 0.0.into()),
        }
    }

    pub fn visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Sets the x position of the cursor
    ///
    /// Single line inputs don't have a y position
    pub fn position(&mut self, position: impl Into<Pixels>) {
        self.position = point(position.into(), self.position.y.into());
    }

    pub fn move_left(&mut self, cx: &mut ViewContext<Self>) {
        // TODO: Clamp position to 0
        if self.position.x > 0.0.into() {
            self.position.x -= 1.0.into();
            // self.update_buffer_gap(cx);
        }

        cx.notify();

        // println!("Cursor moved left. Cursor position: {}", self.position.x);
    }

    // pub fn move_right(&mut self, cx: &mut ViewContext<Self>) {
    //     // TODO: Clamp position to buffer length
    //     let buffer_len = self.buffer.read(cx).text.len();
    //     if self.position < buffer_len {
    //         self.position += 1;
    //         self.update_buffer_gap(cx);
    //     }

    //     cx.notify();

    //     println!("Cursor moved right. Cursor position: {}", self.position);
    // }

    // fn update_buffer_gap(&self, cx: &mut ViewContext<Self>) {
    //     let cursor_position = self.position;

    //     self.buffer.update(cx, move |buffer, _cx| {
    //         buffer.move_gap(cursor_position);
    //     });
    // }
}

impl Render for Cursor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .absolute()
            .bg(hsla(0.0, 0.0, 0.0, 0.0))
            .when(self.visible, |then| then.bg(hsla(0.0, 0.0, 0.0, 1.0)))
            .w_px()
            .h(px(16.0))
        // .left(px(style.padding.left))
    }
}
