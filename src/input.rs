use gpui::{
    div, hsla, px, AppContext, Context, ElementId, EventEmitter, FocusHandle, FocusableView, Hsla,
    InteractiveElement, IntoElement, Model, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, StyledText, TextStyle, View, ViewContext, VisualContext,
};

pub enum InputEvent {
    Focus,
    Blur,
}

impl EventEmitter<InputEvent> for Input {}

#[derive(Clone)]
struct Buffer {
    text: SharedString,
    cursor_position: usize,
}

impl Buffer {
    fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            cursor_position: 0,
        }
    }
}

#[derive(Clone)]
pub struct Outline {
    color: Hsla,
    width: f32,
}

#[derive(Clone)]
pub struct InputStyle {
    pub background: Hsla,
    pub padding: f32,
    pub margin: f32,
    pub ring: Option<Outline>,
    pub border: Option<Outline>,
    pub border_radius: f32,
    pub text: TextStyle,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            background: hsla(0.0, 0.0, 0.0, 1.0),
            padding: 8.0,
            margin: 8.0,
            ring: None,
            border: None,
            border_radius: 0.0,
            text: TextStyle::default(),
        }
    }
}

pub struct Input {
    id: ElementId,
    focus_handle: FocusHandle,
    buffer: Model<Buffer>,

    placeholder: Option<SharedString>,
    style: InputStyle,
}

impl Input {
    pub fn new(
        cx: &mut ViewContext<Self>,
        id: impl Into<ElementId>,
        value: impl Into<SharedString>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();
        // cx.on_blur(&focus_handle, Self::handle_blur).detach();

        let buffer = cx.new_model(|cx| Buffer::new(value));

        Self {
            id: id.into(),
            focus_handle,
            buffer,
            placeholder: None,
            style: InputStyle::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn style(mut self, style: InputStyle) -> Self {
        self.style = style;
        self
    }

    pub fn value(&self, cx: &ViewContext<Self>) -> SharedString {
        self.buffer.read(cx).text.clone()
    }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Focus);

        self.buffer.update(cx, |buffer, cx| {});
    }
}

impl Render for Input {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let style = &self.style.clone();
        let text = style.text.clone();

        let value = self.value(cx);

        let mut input = div();

        if let Some(style) = &self.style.border {
            input = input.border().border_color(style.color)
        };

        println!("{}", value.clone());

        input
            .id(self.id.clone())
            .flex_none()
            .min_w_48()
            .h_12()
            .bg(hsla(1.0, 1.0, 1.0, 0.05))
            .hover(|this| this.bg(hsla(1.0, 1.0, 1.0, 0.1)))
            .active(|this| this.bg(hsla(1.0, 1.0, 1.0, 0.2)))
            .border()
            .border_color(hsla(1.0, 1.0, 1.0, 0.2))
            .p(px(style.padding))
            .m(px(style.margin))
            .rounded(px(style.border_radius))
            // .bg(style.background)
            .text_color(hsla(1.0, 1.0, 1.0, 1.0))
            // .text_color(text.color)
            .font(text.font_family)
            .text_size(text.font_size)
            .child(value)
    }
}

impl FocusableView for Input {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
