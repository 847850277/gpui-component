use std::time::Duration;

use gpui::{
    actions, div, prelude::FluentBuilder as _, App, AppContext, Axis, ClickEvent, Context, Entity,
    Focusable, InteractiveElement, IntoElement, ParentElement as _, Render, Styled, Window,
};
use gpui_component::{
    button::{Button, ButtonGroup, ButtonVariants},
    checkbox::Checkbox,
    divider::Divider,
    form::{form_field, v_form},
    h_flex,
    input::TextInput,
    switch::Switch,
    v_flex, ActiveTheme, AxisExt, FocusableCycle, Selectable, Sizable, Size, Theme,
};
use serde_json::ser;
use sqlx::mysql::MySqlPoolOptions;

actions!(input_story, [Tab, TabPrev]);

pub struct MysqlFormStory {
    system_input: Entity<TextInput>,
    server_input: Entity<TextInput>,
    username_input: Entity<TextInput>,
    password_input: Entity<TextInput>,
    database_input: Entity<TextInput>,
    //login_button: Entity<Button>,
    layout: Axis,
    size: Size,
    // 新增状态字段
    connection_state: ConnectionState,
    error_message: Option<String>,
}

#[derive(PartialEq)]
enum ConnectionState {
    Idle,
    Connecting,
    Connected,
    Failed,
}

impl super::Story for MysqlFormStory {
    fn title() -> &'static str {
        "MysqlFormStory"
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render + Focusable> {
        Self::view(window, cx)
    }
}

impl MysqlFormStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let system_input = cx.new(|cx| {
            let mut input = TextInput::new(window, cx).cleanable();
            input.set_text("MySQL", window, cx);
            input
        });
        let server_input = cx.new(|cx| TextInput::new(window, cx).placeholder("localhost:3306"));
        let username_input =
            cx.new(|cx| TextInput::new(window, cx).placeholder("Enter username..."));
        let password_input = cx.new(|cx| {
            TextInput::new(window, cx).placeholder("Enter password...")
            //.password()
        });
        let database_input =
            cx.new(|cx| TextInput::new(window, cx).placeholder("Enter database name..."));

        //let login_button = cx.new(|cx| Button::new("login").label("Primary Button"));

        // Button::new("button-1")
        // .primary()
        // .label("Primary Button")
        // .disabled(disabled)
        // .selected(selected)
        // .loading(loading)
        // .when(compact, |this| this.compact())
        // .on_click(Self::on_click),

        Self {
            system_input,
            server_input,
            username_input,
            password_input,
            database_input,
            //login_button,
            layout: Axis::Vertical,
            size: Size::default(),
            connection_state: ConnectionState::Idle,
            error_message: None,
        }
    }

    fn on_click(ev: &ClickEvent, window: &mut Window, cx: &mut App) {
        println!("Button clicked! {:?}", ev);
    }

    fn on_click_with_view(
        view: &MysqlFormStory,
        ev: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        println!("Button clicked! {:?}", ev);
    }
}

impl FocusableCycle for MysqlFormStory {
    fn cycle_focus_handles(&self, _: &mut Window, cx: &mut App) -> Vec<gpui::FocusHandle>
    where
        Self: Sized,
    {
        vec![
            self.system_input.focus_handle(cx),
            self.server_input.focus_handle(cx),
            self.username_input.focus_handle(cx),
            self.password_input.focus_handle(cx),
            self.database_input.focus_handle(cx),
        ]
    }
}

impl Focusable for MysqlFormStory {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.system_input.focus_handle(cx)
    }
}

impl Render for MysqlFormStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("form-story")
            .size_full()
            .p_4()
            .justify_start()
            .gap_3()
            .child(
                h_flex()
                    .gap_3()
                    .flex_wrap()
                    .justify_between()
                    .child(
                        Switch::new("layout")
                            .checked(self.layout.is_horizontal())
                            .label("Horizontal")
                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                if *checked {
                                    this.layout = Axis::Horizontal;
                                } else {
                                    this.layout = Axis::Vertical;
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        ButtonGroup::new("size")
                            .small()
                            .child(
                                Button::new("large")
                                    .selected(self.size == Size::Large)
                                    .child("Large"),
                            )
                            .child(
                                Button::new("medium")
                                    .child("Medium")
                                    .selected(self.size == Size::Medium),
                            )
                            .child(
                                Button::new("small")
                                    .child("Small")
                                    .selected(self.size == Size::Small),
                            )
                            .on_click(cx.listener(|this, selecteds: &Vec<usize>, _, cx| {
                                if selecteds.contains(&0) {
                                    this.size = Size::Large;
                                } else if selecteds.contains(&1) {
                                    this.size = Size::Medium;
                                } else if selecteds.contains(&2) {
                                    this.size = Size::Small;
                                }
                                cx.notify();
                            })),
                    ),
            )
            .child(Divider::horizontal())
            .child(
                v_form()
                    .layout(self.layout)
                    .with_size(self.size)
                    .child(
                        form_field()
                            .no_label_indent()
                            .child("Please provide your MySQL connection details."),
                    )
                    .child(
                        form_field()
                            .label_fn(|_, _| "System")
                            .child(self.system_input.clone()),
                    )
                    .child(
                        form_field()
                            .label("Server")
                            .child(self.server_input.clone())
                            .required(true),
                    )
                    .child(
                        form_field()
                            .label("Username")
                            .when(self.layout.is_vertical(), |this| this.items_start())
                            .child(self.username_input.clone())
                            .description_fn(|_, _| div().child("Enter your MySQL username.")),
                    )
                    .child(
                        form_field()
                            .label("Password")
                            .child(self.password_input.clone())
                            .description("Enter your MySQL password."),
                    )
                    .child(
                        form_field()
                            .label("Database")
                            .child(self.database_input.clone())
                            .description("Enter the database name you want to connect to."),
                    )
                    //.child(form_field().child(Button::new("button-1").success().label("login")))
                    .child(
                        // 为 login 按钮添加点击处理和状态显示
                        form_field().child(
                            Button::new("login")
                                .success()
                                .label(match self.connection_state {
                                    ConnectionState::Idle => "Login",
                                    ConnectionState::Connecting => "Connecting...",
                                    ConnectionState::Connected => "Connected",
                                    ConnectionState::Failed => "Retry Connection",
                                })
                                .loading(self.connection_state == ConnectionState::Connecting)
                                //.disabled(self.connection_state == ConnectionState::Connecting)
                                //.on_click(Self::on_click),
                                .on_click(cx.listener(|this, _, _, cx| {
                                    //let user_name = this.username_input.read(cx).text();
                                    // 获取字段
                                    let system = this.system_input.read(cx).text();
                                    let server = this.server_input.read(cx).text();
                                    let user_name = this.username_input.read(cx).text();
                                    let password = this.password_input.read(cx).text();
                                    let database = this.database_input.read(cx).text();
                                    let connection_string = format!(
                                        "mysql://{}:{}@{}/{}",
                                        user_name, password, server, database
                                    );
                                    println!("connection_string: {}", connection_string);
                                    // TODO 获取mysql 连接池
                                    // 连接数据库
                                    this.connection_state = ConnectionState::Connecting;
                                    //cx.notify();

                                    print!("Connecting to MySQL...");
                                    // 连接成功后，更新状态
                                    this.connection_state = ConnectionState::Connected;

                                    //TODO 跳转home页面
                                    

                                    
                                })),
                            //.on_click(Self::on_click_with_view),
                        ),
                    )
                    .child(
                        form_field().child(
                            Checkbox::new("use-vertical-layout")
                                .label("Vertical layout")
                                .checked(self.layout.is_vertical())
                                .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                    this.layout = if *checked {
                                        Axis::Vertical
                                    } else {
                                        Axis::Horizontal
                                    };
                                    cx.notify();
                                })),
                        ),
                    ),
            )
    }
}
