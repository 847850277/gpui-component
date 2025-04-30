use gpui::{px, size, App, AppContext, Application, Bounds, WindowBounds, WindowOptions};
use story_copy::accordion::AccordionStory;

//测试用例展示界面 需要在main函数中运行
fn main() {
    let app = Application::new();
    app.run(|cx: &mut App| {
        // Initialize the gpui-component library
        gpui_component::init(cx);
        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1600.), px(1200.0)), cx);
        let option = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };

        cx.open_window(option, |window, cx| {
            cx.new(|inner_cx| AccordionStory::new(window, inner_cx))
        })
        .unwrap();
    });
}
