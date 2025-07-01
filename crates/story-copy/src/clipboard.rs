use crate::section::section;
use gpui::{
    div, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, SharedString, Styled, Window,
};
use gpui_component::clipboard::Clipboard;
use gpui_component::label::Label;
use gpui_component::link::Link;
use gpui_component::{v_flex, ContextModal};

pub struct ClipboardStory {
    focus_handle: FocusHandle,
    masked: bool,
}

impl super::Story for ClipboardStory {
    fn title() -> &'static str {
        "Clipboard"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl ClipboardStory {
    pub(crate) fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            masked: false,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Focusable for ClipboardStory {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ClipboardStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().gap_6().child(
            section("Copy to Clipboard")
                .max_w_md()
                .child(
                    Clipboard::new("clipboard1")
                        .content(|_, _| Label::new("Click icon to copy"))
                        .value_fn({
                            let view = cx.entity().clone();
                            move |_, cx| {
                                SharedString::from(format!("masked :{}", view.read(cx).masked))
                            }
                        })
                        .on_copied(|value, window, cx| {
                            window.push_notification(format!("Copied value: {}", value), cx)
                        }),
                )
                .child(
                    Clipboard::new("clipboard2")
                        .content(|_, _| {
                            Link::new("link1")
                                .href("https://github.com")
                                .child("GitHub")
                        })
                        .value("https://github.com")
                        .on_copied(|value, window, cx| {
                            window.push_notification(format!("Copied value: {}", value), cx)
                        }),
                ),
        )
    }
}
