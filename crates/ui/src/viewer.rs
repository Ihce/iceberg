use gpui::{ScrollDirection, ScrollView, Scrollable, div, prelude::*, rgb};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Row {
    off: u64,
    size: u8,
    bytes: String,
    text: String,
    ok: bool,
}

pub struct Viewer {
    rows: Vec<Row>,
    scroll: Scrollable, // gpui's built‑in scroll state
}

impl Viewer {
    pub fn load(path: &str) -> Self {
        let data = std::fs::read_to_string(path).expect("scan.jsonl");
        let rows = data
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        Self {
            rows,
            scroll: Scrollable::new(),
        }
    }
}

impl gpui::Render for Viewer {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        // simple manual table (offset | bytes | asm)
        let row_h = 18.0;
        let total_h = self.rows.len() as f32 * row_h;

        ScrollView::new(&mut self.scroll)
            .direction(ScrollDirection::Vertical)
            .content_size((gpui::px(0.0), gpui::px(total_h)))
            .builder(div().child_rows(&self.rows, |row| gpui::px(row_h), |row| render_row(row), cx))
    }
}

// helper: render one row element
fn render_row(row: &Row) -> impl gpui::IntoElement {
    let color = if row.ok { rgb(0xffffff) } else { rgb(0xff8080) };
    div()
        .flex()
        .size_full()
        .text_color(color)
        .child(
            div()
                .width(gpui::px(80.0))
                .monospace(format!("{:08x}", row.off)),
        )
        .child(div().width(gpui::px(140.0)).monospace(&row.bytes))
        .child(div().monospace(&row.text))
}
