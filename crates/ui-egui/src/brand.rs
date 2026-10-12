//! Chromatic's minimal monochrome aperture mark, drawn natively in egui.
//! The vector drawing scales cleanly without loading a legacy PhotoCraft icon.
use egui::{Color32, Context, Id, Rect, Stroke, Ui, pos2, vec2};

fn rect_id() -> Id {
    Id::new("chromatic-brand-mark-rect")
}

/// Paint the Chromatic mark in a square within the title bar.
pub fn paint_mark(ui: &Ui, r: Rect) {
    let painter = ui.painter();
    let side = r.width().min(r.height());
    let square = Rect::from_center_size(r.center(), vec2(side, side));
    let unit = side / 100.0;
    let corner = egui::CornerRadius::same((22.0 * unit).round().clamp(0.0, 255.0) as u8);
    painter.rect_filled(square, corner, Color32::from_rgb(23, 23, 23));
    let center = square.center();
    let white = Color32::from_rgb(245, 245, 245);
    painter.circle_stroke(center, 29.0 * unit, Stroke::new(3.5 * unit, white));
    painter.circle_filled(center, 7.5 * unit, white);
    for (ax, ay, bx, by) in [(50.0, 21.0, 50.0, 43.0), (75.4, 64.6, 56.4, 53.7), (24.6, 64.6, 43.6, 53.7)] {
        let p = |x: f32, y: f32| pos2(square.left() + x * unit, square.top() + y * unit);
        painter.line_segment([p(ax, ay), p(bx, by)], Stroke::new(3.7 * unit, white));
    }
    ui.ctx().data_mut(|d| d.insert_temp(rect_id(), r));
}

#[cfg(test)]
pub fn mark_rect(ctx: &Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(rect_id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn title_bar_paints_the_chromatic_mark() {
        let ctx = Context::default();
        crate::PhotocraftApp::setup_context(&ctx, crate::theme::ThemeKind::ALL[0]);
        let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        app.custom_titlebar = true;
        let output = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
        assert!(output.shapes.len() > 0);
        assert!(mark_rect(&ctx).is_some());
    }
}
