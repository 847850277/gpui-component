use gpui::{Context, SharedString};

pub struct AppTitleBar {
    title: SharedString,
}

impl AppTitleBar {
    pub fn new(title: SharedString, cx: &mut Context<Self>) -> Self {
        Self { title }
    }
}
