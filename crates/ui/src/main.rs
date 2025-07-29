use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};

mod viewer;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |win, cx| {
                win.set_window_title("Superset Viewer – pure gpui");
                cx.new(|_| viewer::Viewer::load("out.jsonl"))
            },
        )
        .unwrap();
    });
}
