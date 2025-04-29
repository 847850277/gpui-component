use app_title_bar::AppTitleBar;
use gpui::{
    div, AnyView, App, Context, Entity, Focusable, IntoElement, ParentElement, Render,
    SharedString, Styled, Window,
};
use gpui_component::{v_flex, Root};
use crate::accordion::AccordionStory;

pub mod accordion;
pub mod app_title_bar;

pub trait Story: Focusable + Render {
    fn title() -> &'static str;

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable>;
}

pub struct StoryRoot {
    title_bar: Entity<AppTitleBar>,
    view: AnyView,
}

impl StoryRoot {
    pub fn new(view: impl Into<AnyView>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        todo!()
        //Self { view: view.into() }
    }
}

impl Render for  StoryRoot {

    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let drawer_layer = Root::render_drawer_layer(window, cx);
        let modal_layer = Root::render_modal_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            .children(drawer_layer)
            .children(modal_layer)
            .child(div().absolute().top_8().children(notification_layer))
    }
}

pub struct SingleStoryRoot {
    view: AnyView,
}

impl SingleStoryRoot {
    pub fn new(view: impl Into<AnyView>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { view: view.into() }
    }
}

impl Render for SingleStoryRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let drawer_layer = Root::render_drawer_layer(window, cx);
        let modal_layer = Root::render_modal_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        div()
            .size_full()
            .child(
                v_flex()
                    .size_full()
                    .child(div().flex_1().overflow_hidden().child(self.view.clone())),
            )
            .children(drawer_layer)
            .children(modal_layer)
            .child(div().absolute().top_8().children(notification_layer))
    }
}
