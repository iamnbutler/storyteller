use gpui::{
    div, hsla, prelude::FluentBuilder, px, AppContext, Context, Edges, ElementId, EventEmitter,
    FocusHandle, FocusableView, Hsla, InteractiveElement, IntoElement, Model, MouseButton,
    ParentElement, Render, SharedString, StatefulInteractiveElement, Styled, StyledText, TextStyle,
    View, ViewContext, VisualContext,
};

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
    // There is a better way to do this,
    // but I don't know how it works yet.
    previous_focus: Option<FocusHandle>,
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
        let previous_focus = cx.focused();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();
        cx.on_blur(&focus_handle, Self::handle_blur).detach();

        let buffer = cx.new_model(|cx| Buffer::new(value));

        Self {
            id: id.into(),
            focus_handle,
            previous_focus,
            buffer,
            placeholder: None,
            style: InputStyle::default(),
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
        self.buffer.read(cx).text.clone()
    }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        self.previous_focus == cx.focused();

        cx.emit(InputEvent::Focus);
        self.buffer.update(cx, |buffer, cx| {});
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Blur);

        if let Some(previous_focus) = self.previous_focus.clone() {
            cx.focus(&previous_focus)
        };

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
        let text = style.text.clone();
        let value = self.value(cx);

        let padding_inset = 1.0;
        let padding = if let Some(ring) = style.ring {
            ring.width + padding_inset
        } else {
            2.0 + padding_inset
        };

        let height = 32.0;
        let calculated_height = height - padding * 2.0;

        let width = 128.0;
        let calculated_width = width - padding * 2.0;

        let mut input = div()
            .id(self.id.clone())
            // TODO: Inputs should have unique group ids
            .group("input")
            .relative()
            .flex()
            .h(px(calculated_height))
            // TODO: Width should be dynamic
            // need to be able to read the width of the input
            .w(px(calculated_width))
            .overflow_hidden();

        let current_style = input.style();

        // == Debug ===
        if (current_style.size.width.is_some() || current_style.size.height.is_some()) {
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
            .p(px(padding_inset))
            .border_2()
            .border_color(transparent())
            .when_some(style.ring, |this, ring| {
                this.when(ring.width > 0.0, |this| this)
                    .border_color(ring.color)
                    .rounded(px(ring.radius))
            })
            .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            .on_click(cx.listener(|this, _event, cx| cx.focus_self()))
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
                            .pl(px(style.padding.left))
                            .pr(px(style.padding.right))
                            .pt(px(style.padding.top))
                            .pb(px(style.padding.bottom))
                            .child(value),
                    ),
            )
    }
}

impl FocusableView for Input {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
