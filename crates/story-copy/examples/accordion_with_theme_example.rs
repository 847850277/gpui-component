use gpui::{
    px, size, App, AppContext, Application, Bounds, SharedString, WindowBounds, WindowOptions,
};
use gpui_component::Root;
use story_copy::accordion::AccordionStory;
use story_copy::assets::Assets;
use story_copy::{SingleStoryRoot, StoryRoot};

//增加title bar
fn main() {
    //let app = Application::new();
    let app = Application::new().with_assets(Assets);
    app.run(|cx: &mut App| {
        // Initialize the gpui-component library
        gpui_component::init(cx);
        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1600.), px(1200.0)), cx);
        let option = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        let title = "Accordion Example";
        let title = SharedString::from(title.to_string());
        cx.open_window(option, |window, cx| {
            // 创建视图
            let view = AccordionStory::view(window, cx);
            //let root = cx.new(|cx| SingleStoryRoot::new(view, window, cx));
            let root = cx.new(|cx| StoryRoot::new(title.clone(), view, window, cx));
            cx.new(|cx| Root::new(root.into(), window, cx))
            //cx.new(|inner_cx| AccordionStory::new(window, inner_cx))
        })
        .expect("failed to open window");
    });
}
