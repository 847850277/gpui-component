use gpui::{
    img, App, AppContext, ClickEvent, ElementId, Entity, FocusHandle, Focusable, ParentElement,
    Render, Styled, Window,
};
use gpui_component::{button::Button, v_flex, SvgImg};

use crate::section::section;

const SVG_ITEMS: &[&str] = &[
    include_str!("./fixtures/google.svg"),
    include_str!("./fixtures/color-wheel.svg"),
];

pub struct ImageStory {
    svg_index: usize,
    focus_handle: FocusHandle,
}

impl ImageStory {
    pub fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            svg_index: 0,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn svg_img(&self, id: impl Into<ElementId>) -> SvgImg {
        SvgImg::new(id, SVG_ITEMS[self.svg_index].as_bytes())
    }
}

impl super::Story for ImageStory {
    fn title() -> &'static str {
        "Image"
    }

    fn new_view(
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> gpui::Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl Focusable for ImageStory {
    fn focus_handle(&self, cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ImageStory {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .gap_4()
            .size_full()
            .child(
                Button::new("switch")
                    .label("Switch SVG")
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.svg_index += 1;
                        if this.svg_index >= SVG_ITEMS.len() {
                            this.svg_index = 0;
                        }
                        cx.notify();
                    })),
            )
            .child(section("SVG 160px").child(self.svg_img("logo1").size_40().flex_grow()))
            .child(section("SVG 80px").child(self.svg_img("logo3").size_20().flex_grow()))
            .child(section("SVG 48px").child(self.svg_img("logo4").size_12().flex_grow()))
            .child(
                section("SVG from img 40px").child(
                    // 不知道为啥，现在加载网络资源是不能显示出来的
                    img("https://pub.lbkrs.com/files/202503/vEnnmgUM6bo362ya/sdk.svg").h_24(),
                ),
            )
    }
}
