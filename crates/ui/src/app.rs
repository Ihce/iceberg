use gpui::{div, prelude::*, IntoElement, Render};
use gpui_component::dock::{Dock, DockItem};
use panels::HelloPanel; // the panel we just wrote

pub struct IcebergApp;

impl Render for IcebergApp {
    fn render(
        &mut self,
        _win: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        // 1.  A root Dock container…
        Dock::new()
            // 2.  Seed it with a single panel
            .with_item(
                DockItem::new("Welcome", |_cx| HelloPanel), // title and view‑builder
            )
            // 3.  And split a second panel to the right (25 % width)
            .split_right(
                DockItem::new("Second pane", |_cx| {
                    div().text("🚀 Ready for content").p_4()
                }),
                0.25, // ratio
            )
    }
}
