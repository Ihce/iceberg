use gpui::{div, prelude::*, IntoElement, Render};

/// A very small demo panel that just shows a label.
pub struct HelloPanel;

impl Render for HelloPanel {
    fn render(
        &mut self,
        _win: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div().text("👋 Hello from the dock!").p_4()
    }
}
