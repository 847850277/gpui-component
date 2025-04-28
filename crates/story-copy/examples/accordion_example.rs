use gpui::{px, size, App, AppContext, Application, Bounds, WindowBounds, WindowOptions};
use story_copy::accordion::AccordionStory;

// TODO 测试用例展示界面 需要在main函数中运行
fn main() {
    let mut app = Application::new();

    app.run(|cx: &mut App| {
        // Initialize the gpui-component library
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|inner_cx| AccordionStory::new(window, inner_cx)),
        )
        .unwrap();
    });
}
