use gpui::{prelude::*, *};

use crate::cursor::Cursor;

actions!(focus, [MoveLeft, MoveRight]);

fn transparent() -> Hsla {
    hsla(0.0, 0.0, 0.0, 0.0)
}

pub enum InputEvent {
    Focus,
    Blur,
}

impl EventEmitter<InputEvent> for Input {}

#[derive(Clone)]
struct Buffer {
    text: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl Buffer {
    fn new(text: impl Into<String>) -> Self {
        let text: Vec<char> = text.into().chars().collect();
        let gap_start = text.len();
        let gap_end = text.len();

        Self {
            text,
            gap_start,
            gap_end,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            text: Vec::with_capacity(capacity),
            gap_start: 0,
            gap_end: 0,
        }
    }

    fn move_gap(&mut self, new_gap_start: usize) {
        if new_gap_start < self.gap_start {
            // Move gap left
            let distance = self.gap_start - new_gap_start;
            for _ in 0..distance {
                self.gap_end -= 1;
                self.gap_start -= 1;
                self.text.swap(self.gap_end, self.gap_start);
            }
        } else if new_gap_start > self.gap_start {
            // Move gap right
            let distance = new_gap_start - self.gap_start;
            for _ in 0..distance {
                self.text.swap(self.gap_end, self.gap_start);
                self.gap_end += 1;
                self.gap_start += 1;
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.gap_start > 0 {
            self.move_gap(self.gap_start - 1);
        }
    }

    pub fn move_right(&mut self) {
        if self.gap_end < self.text.len() {
            self.move_gap(self.gap_start + 1);
        }
    }
    pub fn move_to_start(&mut self) {
        self.move_gap(0);
    }

    pub fn move_to_end(&mut self) {
        self.move_gap(self.text.len() - (self.gap_end - self.gap_start));
    }

    fn insert(&mut self, cx: &mut ModelContext<Self>, c: char) {
        // Ensure the gap has space.
        if self.gap_start == self.gap_end {
            self.expand_gap();
        }

        self.text[self.gap_start] = c;
        self.gap_start += 1;

        cx.notify();
    }

    fn backspace(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.text[self.gap_start] = ' '; // Optional: clear the character for debugging visibility
        }
    }

    fn delete(&mut self) {
        if self.gap_end < self.text.len() {
            self.text[self.gap_end] = ' '; // Optional: clear the character for cleanliness
            self.gap_end += 1;
        }
    }

    fn expand_gap(&mut self) {
        let additional_gap_size = self.text.len().max(1); // Double the size or add 1 if it's empty
        let mut new_text = Vec::with_capacity(self.text.len() + additional_gap_size);

        let (left, right) = self.text.split_at(self.gap_start);
        new_text.extend_from_slice(left);
        new_text.extend(vec![' '; additional_gap_size]);
        new_text.extend_from_slice(&right[self.gap_end - self.gap_start..]);

        self.gap_end += additional_gap_size;
        self.text = new_text;
    }

    fn to_string(&self) -> String {
        self.text[0..self.gap_start]
            .iter()
            .chain(self.text[self.gap_end..].iter())
            .collect()
    }
}

impl Into<SharedString> for Buffer {
    fn into(self) -> SharedString {
        self.to_string().into()
    }
}

#[derive(Clone)]
pub struct Outline {
    pub color: Hsla,
    pub radius: f32,
    pub width: f32,
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            color: hsla(0.0, 0.0, 0.46, 1.0),
            radius: 2.0,
            width: 1.0,
        }
    }
}

impl Outline {
    pub fn new(color: Hsla) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

#[derive(Clone)]
pub struct InputStyle {
    pub background: Hsla,
    pub padding: Edges<f32>,
    pub margin: Edges<f32>,
    pub ring: Option<Outline>,
    pub border: Outline,
    pub text: TextStyle,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0),
            padding: Edges {
                top: 0.0,
                bottom: 0.0,
                left: 4.0,
                right: 4.0,
            },
            margin: Edges::all(0.0),
            ring: None,
            border: Outline::new(hsla(0.0, 0.0, 0.31, 0.4)),
            text: TextStyle::default(),
        }
    }
}

pub struct Input {
    id: ElementId,
    focus_handle: FocusHandle,
    text: Option<ShapedLine>,
    cursor: View<Cursor>,
    cursor_visible: bool,
    cursor_position: Point<Pixels>,
    placeholder: Option<SharedString>,
    style: InputStyle,
}

impl Input {
    pub fn new(
        cx: &mut ViewContext<Self>,
        id: impl Into<ElementId>,
        value: impl Into<String>,
    ) -> Self {
        cx.bind_keys([KeyBinding::new("left", MoveLeft, Some("input"))]);
        cx.bind_keys([KeyBinding::new("right", MoveRight, Some("input"))]);

        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();
        cx.on_blur(&focus_handle, Self::handle_blur).detach();

        let cursor = cx.new_view(|cx| Cursor::new(cx));

        Self {
            id: id.into(),
            focus_handle,
            text: None,
            cursor,
            cursor_visible: false,
            placeholder: None,
            style: InputStyle::default(),
            cursor_position: point(0.0.into(), 0.0.into()),
        }
    }

    pub fn set_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn set_style(mut self, style: InputStyle) -> Self {
        self.style = style;
        self
    }

    pub fn value(&self, cx: &ViewContext<Self>) -> SharedString {
        if let Some(line) = self.text.as_ref() {
            line.text.clone()
        } else {
            "".into()
        }
    }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Focus);
        // self.buffer.update(cx, |buffer, cx| {});
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Blur);
        cx.notify();
    }

    pub fn is_focused(&self, cx: &ViewContext<Self>) -> bool {
        cx.focused() == Some(self.focus_handle.clone())
    }
}

impl Render for Input {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // == Style ==
        let mut style = self.style.clone();
        // let is_focused = self.is_focused(cx);

        let value = if self.value(cx).len() == 0 {
            println!(
                "Placeholder: {:?}",
                self.placeholder.clone().unwrap_or_else(|| "".into())
            );
            style.text.color = hsla(0.0, 0.0, 0.67, 1.0);
            self.placeholder.clone().unwrap_or_else(|| "".into())
        } else {
            println!("Value: {:?}", self.value(cx));
            style.text.color = TextStyle::default().color;
            self.value(cx)
        };

        let text = style.text.clone();

        // == Size ==
        let padding_inset = 1.0;
        let padding = if let Some(ring) = style.ring {
            ring.width + padding_inset
        } else {
            2.0 + padding_inset
        };

        let height = 32.0;
        let calculated_height = height - padding * 2.0;

        let width = 188.0;
        let calculated_width = width - padding * 2.0;

        // let cursor = self.cursor.clone();
        // let cursor_2 = self.cursor.clone();
        let cursor_current_position = self.cursor.clone().read(cx).position.x;
        if let Some(line) = self.text.clone() {
            line.text
        } else {
            "".into()
        };

        let mut input = div()
            .id(self.id.clone())
            // TODO: Inputs should have unique group ids
            .group("input")
            .track_focus(&self.focus_handle)
            .key_context("input")
            // .on_action(cx.listener(move |_, _action: &MoveRight, cx| {
            //     let cursor_clone = cursor.clone();
            //     cursor_clone.update(cx, |cursor, cx| cursor.move_right(cx))
            // }))
            // .on_action(cx.listener(move |_, _action: &MoveLeft, cx| {
            //     let cursor_clone_2 = cursor_2.clone();
            //     cursor_clone_2.update(cx, |cursor, cx| cursor.move_left(cx))
            // }))
            // .on_key_down(cx.listener(move |_, event: &KeyDownEvent, cx| {
            //     let is_printable = event
            //         .keystroke
            //         .key
            //         .chars()
            //         .all(|c| c.is_ascii_graphic() || c.is_whitespace());
            //     if is_printable {
            //         let buffer_clone = text.clone();
            //         buffer_clone.update(cx, |buffer, cx| {
            //             let key = event.keystroke.key.clone();
            //             let mut char: Option<char> = "".chars().next();
            //             if key == "space" {
            //                 char = " ".chars().next()
            //             } else {
            //                 char = key.chars().next()
            //             }
            //             if let Some(char) = char {
            //                 buffer.insert(cx, char);
            //             } else {
            //                 println!("No char found for {:?}", key);
            //             }
            //         });
            //         cx.notify();
            //         println!("Printable key down on parent {:?}", event)
            //     } else {
            //         // Non-printable key pressed, handle accordingly or ignore.
            //         println!("Non-printable key down ignored {:?}", event)
            //     }
            // }))
            .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            .on_click(cx.listener(|this, _event, cx| cx.focus_self()))
            .relative()
            .flex()
            .h(px(calculated_height))
            // TODO: Width should be dynamic
            // need to be able to read the width of the input
            .w(px(calculated_width))
            .overflow_hidden();

        let current_style = input.style();

        // == Debug ===
        if current_style.size.width.is_some() || current_style.size.height.is_some() {
            print!("Size: ");
            if let Some(current_width) = current_style.size.width {
                print!("width: {:?} ", current_width);
            }
            if let Some(current_height) = current_style.size.height {
                print!("height: {:?} ", current_height);
            }
            println!();
        } else {
            println!("Size: None");
        }

        println!(
            "Focus: Current: {:?}, Input: {:?}, Focused: {:?}",
            cx.focused(),
            self.focus_handle,
            self.is_focused(cx)
        );

        println!("Cursor visible: {:?}", self.cursor_visible);

        // println!("Input focus handle: {:?}", self.focus_handle);
        // println!("Current focus handler: {:?}", cx.focused());

        match self.is_focused(cx) {
            true => {
                style.ring = Some(Outline::new(hsla(0.6, 0.67, 0.46, 1.0)));
            }
            false => {
                style.ring = None;
            }
        }

        input
            .cursor(CursorStyle::IBeam)
            .p(px(padding_inset))
            .border_2()
            .border_color(transparent())
            .when_some(style.ring, |this, ring| {
                this.when(ring.width > 0.0, |this| this)
                    .border_color(ring.color)
                    .rounded(px(ring.radius))
            })
            // .on_key_down(cx.listener(
            //     |this,
            //      KeyDownEvent {
            //          keystroke: Keystroke::parse("right"),
            //          is_held: false,
            //      },
            //      cx| {},
            // ))
            .child(
                div()
                    .id("input_inner")
                    .absolute()
                    .flex()
                    .h(px(calculated_height - padding_inset * 2.0))
                    .w(px(calculated_width - padding_inset * 2.0))
                    .top(px(-padding_inset))
                    .left(px(-padding_inset))
                    .items_center()
                    .bg(style.background)
                    .when(style.border.width > 0.0, |this| this.border())
                    .border_color(style.border.color)
                    .rounded(px(style.border.radius))
                    .overflow_hidden()
                    .bg(style.background)
                    .text_color(text.color)
                    .font(text.font_family)
                    .text_size(text.font_size)
                    .group_hover("input", |this| this.border_color(hsla(0.0, 0.0, 0.31, 1.0)))
                    .child(
                        div()
                            .relative()
                            .pl(px(style.padding.left))
                            .pr(px(style.padding.right))
                            .pt(px(style.padding.top))
                            .pb(px(style.padding.bottom))
                            .child(value),
                    )
                    .child(
                        // Cursor - this won't actually be implemented this way.
                        // This just let's us show off the cursor and blinking
                        div()
                            .absolute()
                            .bg(hsla(0.0, 0.0, 0.0, 0.0))
                            .when(self.is_focused(cx), |then| {
                                then.bg(hsla(0.0, 0.0, 0.0, 1.0))
                            })
                            .w_px()
                            .h(px(calculated_height - 6.0))
                            // This is dumb, just doing this to see the cursor move for now
                            .left(px(style.padding.left) + cursor_current_position * px(6.0)),
                    ),
            )
    }
}

impl FocusableView for Input {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
