use gpui::{
    div, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};
use gpui_component::button::ButtonCustomVariant;
use gpui_component::checkbox::Checkbox;
use gpui_component::{h_flex, indigo, indigo_800, v_flex, white, ActiveTheme};

pub struct ButtonStory {
    focus_handle: FocusHandle,
    disabled: bool,
    loading: bool,
    selected: bool,
    compact: bool,
    toggle_multiple: bool,
}

impl ButtonStory {
    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            disabled: false,
            loading: false,
            selected: false,
            compact: false,
            toggle_multiple: false,
        })
    }
}

impl Focusable for ButtonStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl super::Story for ButtonStory {
    fn title() -> &'static str {
        "Button"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl Render for ButtonStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let disabled = self.disabled;
        let loading = self.loading;
        let selected = self.selected;
        let compact = self.compact;
        let toggle_multiple = self.toggle_multiple;

        let custom_variant = ButtonCustomVariant::new(cx)
            .color(if cx.theme().mode.is_dark() {
                indigo_800()
            } else {
                indigo(600)
            })
            .foreground(white())
            .border(if cx.theme().mode.is_dark() {
                indigo_800()
            } else {
                indigo(600)
            })
            .hover(if cx.theme().mode.is_dark() {
                indigo(900)
            } else {
                indigo(700)
            })
            .active(if cx.theme().mode.is_dark() {
                indigo(950)
            } else {
                indigo(700)
            });

        v_flex().gap_6().child(
            h_flex()
                .gap_3()
                .child(
                    Checkbox::new("disable-button")
                        .label("Disabled")
                        .checked(self.disabled),
                )
                .child(
                    Checkbox::new("loading-button")
                        .label("Loading")
                        .checked(self.loading),
                )
                .child(
                    Checkbox::new("selected-button")
                        .label("Selected")
                        .checked(self.selected),
                )
                .child(
                    Checkbox::new("compact-button")
                        .label("Compact")
                        .checked(self.compact),
                )
                .child(
                    Checkbox::new("shadow-button")
                        .label("Shadow")
                        .checked(cx.theme().shadow),
                ),
        )
    }
}
