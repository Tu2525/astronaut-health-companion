//! Painter-drawn icons. egui's bundled fonts lack ◐ ◆ and the like, so every glyph
//! is a shape: no icon font, no new dependency, identical on native and WASM.
//! Colours are passed in by the caller (tokens live in `theme.rs`).

use ahc_core::Status;
use eframe::egui::{self, pos2, vec2, Color32, Pos2, Sense, Shape, Stroke, Ui};

const SIZE: f32 = 16.0;

fn poly(pts: Vec<Pos2>, c: Color32) -> Shape {
    Shape::convex_polygon(pts, c, Stroke::NONE)
}

fn arc(c: Pos2, r: f32, from: f32, to: f32) -> Vec<Pos2> {
    (0..=14)
        .map(|i| {
            let a = from + (to - from) * i as f32 / 14.0;
            pos2(c.x + r * a.cos(), c.y + r * a.sin())
        })
        .collect()
}

/// Status glyphs from `docs/contract.md` section 2: ● ◐ ▲ ◆ ○.
pub fn status(ui: &mut Ui, s: Status, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(SIZE, SIZE), Sense::hover());
    let (c, r) = (rect.center(), 6.5_f32);
    let p = ui.painter();
    let stroke = Stroke::new(1.5_f32, color);
    match s {
        Status::Nominal => {
            p.circle_filled(c, r, color);
        }
        Status::Watch => {
            p.circle_stroke(c, r, stroke);
            let half = arc(
                c,
                r,
                -std::f32::consts::FRAC_PI_2,
                std::f32::consts::FRAC_PI_2,
            );
            p.add(poly(half, color));
        }
        Status::Caution => {
            p.add(poly(
                vec![
                    pos2(c.x, c.y - r),
                    pos2(c.x + r, c.y + r),
                    pos2(c.x - r, c.y + r),
                ],
                color,
            ));
        }
        Status::Urgent => {
            p.add(poly(
                vec![
                    pos2(c.x, c.y - r),
                    pos2(c.x + r, c.y),
                    pos2(c.x, c.y + r),
                    pos2(c.x - r, c.y),
                ],
                color,
            ));
        }
        Status::Unknown => {
            p.circle_stroke(c, r, stroke);
        }
    }
}

/// A small icon for a registry indicator (by id), or a ring when there is none.
pub fn indicator(ui: &mut Ui, id: &str, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(SIZE, SIZE), Sense::hover());
    let (c, p) = (rect.center(), ui.painter());
    let stroke = Stroke::new(1.5_f32, color);
    let at = |x: f32, y: f32| pos2(c.x + x, c.y + y);
    match id {
        "heart_rate" | "bp_systolic" | "bp_diastolic" => {
            // Heart: two lobes and a point.
            p.circle_filled(at(-3.0, -2.5), 3.5, color);
            p.circle_filled(at(3.0, -2.5), 3.5, color);
            p.add(poly(
                vec![at(-6.4, -1.5), at(6.4, -1.5), at(0.0, 6.5)],
                color,
            ));
        }
        "spo2" | "blood_glucose" => {
            // Drop: round bottom, pointed top.
            p.circle_filled(at(0.0, 2.0), 4.5, color);
            p.add(poly(
                vec![at(0.0, -7.0), at(4.2, 0.5), at(-4.2, 0.5)],
                color,
            ));
        }
        "body_temp" => {
            // Thermometer: stem and bulb.
            p.rect_filled(
                egui::Rect::from_center_size(at(0.0, -2.0), vec2(3.0, 9.0)),
                1.0,
                color,
            );
            p.circle_filled(at(0.0, 4.0), 3.5, color);
        }
        "sleep_hours" => {
            // Moon: a thick open arc reads as a crescent at this size.
            let a = arc(at(1.0, 0.0), 5.5, 0.9, 0.9 + 4.4);
            p.add(Shape::line(a, Stroke::new(3.0_f32, color)));
        }
        "mood" => {
            p.circle_stroke(c, 6.5, stroke);
            p.circle_filled(at(-2.3, -1.8), 1.0, color);
            p.circle_filled(at(2.3, -1.8), 1.0, color);
            p.add(Shape::line(
                arc(at(0.0, 0.5), 3.5, 0.5, std::f32::consts::PI - 0.5),
                stroke,
            ));
        }
        "stress" => {
            p.add(Shape::line(
                vec![
                    at(-7.0, 0.0),
                    at(-3.5, 0.0),
                    at(-1.5, -6.0),
                    at(1.5, 6.0),
                    at(3.5, 0.0),
                    at(7.0, 0.0),
                ],
                stroke,
            ));
        }
        _ => {
            p.circle_stroke(c, 4.5, stroke);
        }
    }
}
