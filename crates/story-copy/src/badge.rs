use gpui::{
    div, img, App, AppContext, Context, Entity, FocusHandle, Focusable, ParentElement, Render,
    Styled, Window,
};
use gpui_component::{badge::Badge, v_flex, ActiveTheme, Icon, IconName, Sizable};

use crate::section::section;

pub struct BadgeStory {
    focus_handle: FocusHandle,
}

impl BadgeStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl super::Story for BadgeStory {
    fn title() -> &'static str {
        "Badge"
    }

    fn new_view(
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> gpui::Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl Focusable for BadgeStory {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for BadgeStory {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .gap_4()
            .child(
                img("https://is1-ssl.mzstatic.com/image/thumb/avICmr1PbBRB-PAeplGreA/1378x774.jpg")
                    .size_10()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded_full(),
            )
            .child(
                section("Badge on Icon")
                    .max_w_md()
                    .child(
                        Badge::new()
                            .count(3)
                            .child(Icon::new(IconName::Bell).large()),
                    )
                    .child(
                        Badge::new()
                            .count(103)
                            .child(Icon::new(IconName::Inbox).large()),
                    ),
            )
            .child(
                section("Badge on Avatar").max_w_md().child(
                    Badge::new()
                        .count(3)
                        .child(
                            img("https://is1-ssl.mzstatic.com/image/thumb/avICmr1PbBRB-PAeplGreA/1378x774.jpg")
                                .size_10()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_full(),
                        )
                        .child(
                            img("https://avatars.githubusercontent.com/u/28998859?v=4")
                                .size_10()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_full(),
                        ),
                ),
            )
    }
}
