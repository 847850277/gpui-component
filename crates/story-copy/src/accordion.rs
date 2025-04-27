use gpui::{
    div, App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};
use gpui_component::{
    button::{Button, ButtonGroup},
    checkbox::Checkbox,
    h_flex, v_flex, Selectable, Size, StyledExt,
};

pub struct Accordion {
    open_ixs: Vec<usize>,
    size: Size,
    bordered: bool,
    disabled: bool,
    multiple: bool,
    focus_handle: FocusHandle,
}

impl super::Story for Accordion {
    fn title() -> &'static str {
        "Accordion"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl Accordion {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            bordered: true,
            open_ixs: Vec::new(),
            size: Size::default(),
            disabled: false,
            multiple: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn set_size(&mut self, size: Size, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl Focusable for Accordion {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Accordion {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().gap_3().child(
            h_flex()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    ButtonGroup::new("toggle-size")
                        .outline()
                        .compact()
                        .child(
                            Button::new("xsmall")
                                .label("XSmall")
                                .selected(self.size == Size::XSmall),
                        )
                        .child(
                            Button::new("small")
                                .label("Small")
                                .selected(self.size == Size::Small),
                        )
                        .child(
                            Button::new("medium")
                                .label("Medium")
                                .selected(self.size == Size::Medium),
                        )
                        .child(
                            Button::new("large")
                                .label("Large")
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
                .child(
                    Checkbox::new("multiple")
                        .label("Multiple")
                        .checked(self.multiple)
                        .on_click(cx.listener(|this, checked, _, cx| {
                            this.multiple = *checked;
                            cx.notify();
                        })),
                )
                .child(
                    Checkbox::new("disabled")
                        .label("Disabled")
                        .checked(self.disabled)
                        .on_click(cx.listener(|this, checked, _, cx| {
                            this.disabled = *checked;
                            cx.notify();
                        })),
                )
                .child(
                    Checkbox::new("bordered")
                        .label("Bordered")
                        .checked(self.bordered)
                        .on_click(cx.listener(|this, checked, _, cx| {
                            this.bordered = *checked;
                            cx.notify();
                        })),
                ),
        )
    }
}

// test module
// TODO 测试用例展示界面
#[cfg(test)]
mod tests {
    use gpui::{px, size, Application, Bounds, WindowBounds, WindowOptions};

    use super::*;

    #[test]
    fn test_accordion() {
        println!("Accordion test");
        // 展示 render
        //let mut cx = App::new();
        //gpui_component::init(&mut cx);
        let mut app = Application::new();
        app.run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| cx.new(|inner_cx| Accordion::new(window, inner_cx)),
            )
            .unwrap();
        });
    }
}
