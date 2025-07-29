use gpui::{px, size, App, Application, Bounds, WindowBounds, WindowOptions};

mod app; // <‑‑ the view implementation below

fn main() {
    Application::new().run(|cx: &mut App| {
        // 400 × 200 centred window
        let bounds = Bounds::centered(None, size(px(400.0), px(200.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                window.set_title("Iceberg");
                cx.new(|_| app::IcebergApp)
            },
        )
        .unwrap();
    });
}
