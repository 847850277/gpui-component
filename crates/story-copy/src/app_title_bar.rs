use crate::SelectLocale;
use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, AppContext, ClickEvent, Context, Corner,
    Entity, FocusHandle, Hsla, InteractiveElement, IntoElement, MouseButton, ParentElement, Render,
    SharedString, Styled, Subscription, Window,
};
use gpui_component::badge::Badge;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::color_picker::{ColorPicker, ColorPickerEvent};
use gpui_component::popup_menu::PopupMenuExt;
use gpui_component::scroll::ScrollbarShow;
use gpui_component::{
    locale, set_locale, ActiveTheme, ContextModal, IconName, Sizable, Theme, ThemeMode, TitleBar,
};
use std::rc::Rc;

pub struct AppTitleBar {
    title: SharedString,
    theme_color: Option<Hsla>,
    locale_selector: Entity<LocaleSelector>,
    font_size_selector: Entity<FontSizeSelector>,
    theme_color_picker: Entity<ColorPicker>,
    child: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
    _subscriptions: Vec<Subscription>,
}

impl AppTitleBar {
    pub fn new(
        title: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let locale_selector = cx.new(|cx| LocaleSelector::new(window, cx));
        let font_size_selector = cx.new(|cx| FontSizeSelector::new(window, cx));
        if cx.should_auto_hide_scrollbars() {
            Theme::global_mut(cx).scrollbar_show = ScrollbarShow::Scrolling;
        } else {
            Theme::global_mut(cx).scrollbar_show = ScrollbarShow::Hover;
        }
        let theme_color_picker = cx.new(|cx| {
            let mut picker = ColorPicker::new("theme-color-picker", window, cx)
                .small()
                .anchor(Corner::TopRight)
                .icon(IconName::Palette);
            picker.set_value(cx.theme().primary, window, cx);
            picker
        });

        let _subscriptions = vec![cx.subscribe_in(
            &theme_color_picker,
            window,
            |this, _, ev: &ColorPickerEvent, window, cx| match ev {
                ColorPickerEvent::Change(color) => {
                    this.set_theme_color(*color, window, cx);
                }
            },
        )];

        Self {
            title: title.into(),
            theme_color: None,
            locale_selector,
            font_size_selector,
            theme_color_picker,
            child: Rc::new(|_, _| div().into_any_element()),
            _subscriptions,
        }
    }

    fn set_theme_color(
        &mut self,
        color: Option<Hsla>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.theme_color = color;
        if let Some(color) = self.theme_color {
            let theme = cx.global_mut::<Theme>();
            theme.apply_color(color);
            window.refresh();
        }
    }

    pub fn child<F, E>(mut self, f: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut Window, &mut App) -> E + 'static,
    {
        self.child = Rc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    fn change_color_mode(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let mode = match cx.theme().mode.is_dark() {
            true => ThemeMode::Light,
            false => ThemeMode::Dark,
        };

        Theme::change(mode, None, cx);
        self.set_theme_color(self.theme_color, window, cx);
    }
}

impl Render for AppTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();

        TitleBar::new()
            // left side
            .child(div().flex().items_center().child(self.title.clone()))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .gap_2()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child((self.child.clone())(window, cx))
                    .child(self.theme_color_picker.clone())
                    .child(
                        Button::new("theme-mode")
                            .map(|this| {
                                if cx.theme().mode.is_dark() {
                                    this.icon(IconName::Sun)
                                } else {
                                    this.icon(IconName::Moon)
                                }
                            })
                            .small()
                            .ghost()
                            .on_click(cx.listener(Self::change_color_mode)),
                    )
                    .child(self.locale_selector.clone())
                    //.child(self.font_size_selector.clone())
                    .child(
                        Button::new("github")
                            .icon(IconName::GitHub)
                            .small()
                            .ghost()
                            .on_click(|_, _, cx| {
                                cx.open_url("https://github.com/longbridge/gpui-component")
                            }),
                    )
                    .child(
                        div().relative().child(
                            Badge::new().count(notifications_count).max(99).child(
                                Button::new("bell")
                                    .small()
                                    .ghost()
                                    .compact()
                                    .icon(IconName::Bell),
                            ),
                        ),
                    ),
            )
    }
}

pub struct LocaleSelector {
    focus_handle: FocusHandle,
}

impl LocaleSelector {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn on_select_locale(
        &mut self,
        locale: &SelectLocale,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        set_locale(&locale.0);
        window.refresh();
    }
}

impl Render for LocaleSelector {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let locale = locale().to_string();

        div()
            .id("locale-selector")
            .track_focus(&focus_handle)
            .on_action(cx.listener(Self::on_select_locale))
            .child(
                Button::new("btn")
                    .small()
                    .ghost()
                    .icon(IconName::Globe)
                    .popup_menu(move |this, _, _| {
                        this.menu_with_check(
                            "English",
                            locale == "en",
                            Box::new(SelectLocale("en".into())),
                        )
                        .menu_with_check(
                            "简体中文",
                            locale == "zh-CN",
                            Box::new(SelectLocale("zh-CN".into())),
                        )
                    })
                    .anchor(Corner::TopRight),
            )
    }
}

pub struct FontSizeSelector {
    focus_handle: FocusHandle,
}

impl FontSizeSelector {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}
