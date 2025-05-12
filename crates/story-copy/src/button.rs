use crate::section::section;
use gpui::prelude::FluentBuilder;
use gpui::{
    actions, div, App, AppContext, ClickEvent, Context, Entity, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, Styled, Window,
};
use gpui_component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_component::checkbox::Checkbox;
use gpui_component::{
    h_flex, indigo, indigo_800, v_flex, white, ActiveTheme, Disableable, Selectable, Theme,
};

actions!(button_story, [Disabled, Loading, Selected, Compact]);

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

    fn on_click(ev: &ClickEvent, _window: &mut Window, _cx: &mut App) {
        println!("Button clicked! {:?}", ev);
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

        v_flex()
            .on_action(cx.listener(|this, _: &Disabled, _, _| this.disabled = !this.disabled))
            .on_action(cx.listener(|this, _: &Loading, _, _| this.loading = !this.loading))
            .on_action(cx.listener(|this, _: &Selected, _, _| this.selected = !this.selected))
            .on_action(cx.listener(|this, _: &Compact, _, _| this.compact = !this.compact))
            .gap_6()
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        Checkbox::new("disable-button")
                            .label("Disabled")
                            .checked(self.disabled)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.disabled = !view.disabled;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("loading-button")
                            .label("Loading")
                            .checked(self.loading)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.loading = !view.loading;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("selected-button")
                            .label("Selected")
                            .checked(self.selected)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.selected = !view.selected;
                                println!("selected: {}", view.selected);
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("compact-button")
                            .label("Compact")
                            .checked(self.compact)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.compact = !view.compact;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("shadow-button")
                            .label("Shadow")
                            .checked(cx.theme().shadow)
                            .on_click(cx.listener(|_, _, window, cx| {
                                let mut theme = cx.theme().clone();
                                theme.shadow = !theme.shadow;
                                cx.set_global::<Theme>(theme);
                                window.refresh();
                            })),
                    ),
            )
            .child(
                section("Normal Button")
                    .max_w_lg()
                    .child(
                        Button::new("button-1")
                            .primary()
                            .label("Primary Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_action(cx.listener(|this, _: &Compact, _, _| {
                                //this.compact = !this.compact;
                                println!("123");
                            })),
                    )
                    .child(
                        Button::new("button-2")
                            .label("Secondary Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-4")
                            .danger()
                            .label("Danger Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-4-warning")
                            .warning()
                            .label("Warning Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-4-success")
                            .success()
                            .label("Success Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-5-info")
                            .info()
                            .label("Info Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-5-ghost")
                            .ghost()
                            .label("Ghost Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-5-link")
                            .link()
                            .label("Link Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Button::new("button-5-text")
                            .text()
                            .label("Text Button")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
    }
}
