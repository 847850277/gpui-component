use gpui::{App, Entity, Focusable, Render, Window};

pub mod accordion;

pub trait Story: Focusable + Render {
    fn title() -> &'static str;

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable>;
}
