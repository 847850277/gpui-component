use gpui::{div, Context, FocusHandle, Focusable, IntoElement, Render, Window};
use gpui_component::Size;

pub struct Accordion {
    open_ixs: Vec<usize>,
    size: Size,
    border: bool,
    disabled: bool,
    multiple: bool,
    focus_handle: FocusHandle,
}

impl Focusable for Accordion {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Accordion {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
