use crate::section::section;
use gpui::{
    div, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};
use gpui_component::alert::Alert;
use gpui_component::{
    button::{Button, ButtonGroup},
    v_flex, IconName, Selectable, Sizable, Size,
};

pub struct AlertStory {
    size: Size,
    focus_handle: FocusHandle,
}

impl AlertStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            size: Size::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn set_size(&mut self, size: Size, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl super::Story for AlertStory {
    fn title() -> &'static str {
        "Alert"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl Focusable for AlertStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AlertStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // div().child(div().h_1().child("Alert")
        // ).child(div().h_2().child("Displays a callout for user attention."))
        //
        v_flex()
            .gap_4()
            .child(
                ButtonGroup::new("toggle-size")
                    .outline()
                    .compact()
                    .child(
                        Button::new("xsmall")
                            .label("xsmall")
                            .selected(self.size == Size::XSmall),
                    )
                    .child(
                        Button::new("small")
                            .label("small")
                            .selected(self.size == Size::Small),
                    )
                    .child(
                        Button::new("medium")
                            .label("medium")
                            .selected(self.size == Size::Medium),
                    )
                    .child(
                        Button::new("large")
                            .label("large")
                            .selected(self.size == Size::Large),
                    )
                    .on_click(cx.listener(|this, selecteds: &Vec<usize>, window, cx| {
                        let size = match selecteds[0] {
                            0 => Size::XSmall,
                            1 => Size::Small,
                            2 => Size::Medium,
                            3 => Size::Large,
                            _ => unreachable!(),
                        };
                        this.set_size(size, window, cx);
                    })),
            )
            //StorySection
            .child(
                section("Info").max_w_md().child(
                    Alert::info("This is an info alert.")
                        .with_size(self.size)
                        .title("Info message"),
                ),
            )
            .child(
                section("Warning").max_w_md().child(
                    Alert::warning("This is a warning alert.")
                        .with_size(self.size)
                        .title("Warning message"),
                ),
            )
            .child(
                section("Success with Title").max_w_md().child(
                    Alert::success("This is an error alert.")
                        .with_size(self.size)
                        .title("Error message"),
                ),
            )
            .child(
                section("Error").max_w_md().child(
                    Alert::error("This is an error alert.")
                        .with_size(self.size)
                        .title("Error message"),
                ),
            )
            .child(
                section("Custom Icon").max_w_md().child(
                    Alert::info("Custom icon with info alert.")
                        .with_size(self.size)
                        .title("Custom Icon")
                        .icon(IconName::Bell),
                ),
            )

        // .child(Alert::info("This is an info alert.")
        //            .with_size(self.size)
        //            .title("Info message"),)
        // .child(Alert::warning("This is a warning alert.")
        //            .with_size(self.size)
        //            .title("Warning message"),)
        // .child(Alert::success("This is an error alert.")
        //            .with_size(self.size)
        //            .title("Error message"),)
        // .child(Alert::error("This is an error alert.")
        //            .with_size(self.size)
        //            .title("Error message"),)
        // .child(Alert::info("Custom icon with info alert.")
        //            .with_size(self.size)
        //            .title("Custom Icon")
        //     .icon(IconName::Bell))
    }
}
