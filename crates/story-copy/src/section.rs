use gpui::{
    div, rems, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, Styled, Window,
};
use gpui_component::context_menu::ContextMenuExt;
use gpui_component::{h_flex, v_flex, ActiveTheme};

#[derive(IntoElement)]
pub struct StorySection {
    base: Div,
    title: AnyElement,
    children: Vec<AnyElement>,
}

impl StorySection {
    #[allow(unused)]
    pub fn max_w_md(mut self) -> Self {
        self.base = self.base.max_w(rems(48.));
        self
    }

    #[allow(unused)]
    pub fn max_w_lg(mut self) -> Self {
        self.base = self.base.max_w(rems(64.));
        self
    }

    #[allow(unused)]
    pub fn max_w_xl(mut self) -> Self {
        self.base = self.base.max_w(rems(80.));
        self
    }

    #[allow(unused)]
    pub fn max_w_2xl(mut self) -> Self {
        self.base = self.base.max_w(rems(96.));
        self
    }
}

impl Styled for StorySection {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for StorySection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for StorySection {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .mb_5()
            .w_full()
            .child(h_flex().justify_between().gap_4().child(self.title))
            .child(
                v_flex()
                    .p_4()
                    .overflow_x_hidden()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_lg()
                    .items_center()
                    .justify_center()
                    .child(self.base.children(self.children)),
            )
    }
}

impl ContextMenuExt for StorySection {}

pub fn section(title: impl IntoElement) -> StorySection {
    StorySection {
        title: title.into_any_element(),
        base: h_flex()
            .flex_wrap()
            .justify_center()
            .items_center()
            .w_full()
            .gap_4(),
        children: vec![],
    }
}
