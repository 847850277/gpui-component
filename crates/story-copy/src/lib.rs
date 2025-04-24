use gpui::{Focusable, Render};

pub mod accordion;

pub trait Story: Focusable + Render {}
