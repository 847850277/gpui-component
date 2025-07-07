pub struct String1 {
    text: SharedString,
}

impl String1 {
    pub fn new(text: SharedString) -> Self {
        Self { text }
    }

    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            text: "hello1".to_string().into(),
        })
    }
}

impl Render for String1 {
    fn render(&self, cx: &mut App) -> Entity<Self> {
        div(cx, |cx| text(cx, &self.text))
    }
}

fn main() {
    let mut app = App::new();
    let window = Window::new(&mut app);
    let string1 = String1::new("hello1".to_string().into());
    window.add_child(string1.render(&mut app));
    app.run();
}
