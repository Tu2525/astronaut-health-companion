//! ahc-safe: Safe-Mode Console. SAFE-1: prints the demo assessment as a table.
//! No status logic here: everything comes from `ahc_core::assess`.
mod theme;

use ahc_core::{assess, demo, registry, AssessInput, Assessment, Status};
use eframe::egui::{self, pos2, Color32, Label, RichText, Sense, Shape, Stroke, Vec2};

struct App {
    assessment: Assessment,
    now_met_s: f64,
    names: Vec<(String, String)>,
}

impl App {
    fn new() -> Self {
        let input = AssessInput {
            crew_id: demo::CREW_ID.into(),
            events: demo::events(),
            now_met_s: demo::NOW_MET_S,
            kit: demo::kit(),
            registry: None,
        };
        let names = registry().into_iter().map(|i| (i.id, i.name)).collect();
        Self {
            assessment: assess(&input),
            now_met_s: input.now_met_s,
            names,
        }
    }

    fn name_of(&self, id: &str) -> String {
        self.names
            .iter()
            .find(|(i, _)| i == id)
            .map(|(_, n)| n.clone())
            .unwrap_or_else(|| id.into())
    }
}

/// Draws the status icon as a shape: the default font has no ● ◐ ▲ ◆ glyphs.
fn status_icon(ui: &mut egui::Ui, s: Status, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
    let (c, r) = (rect.center(), 6.5_f32);
    let p = ui.painter();
    let stroke = Stroke::new(2.0_f32, color);
    match s {
        Status::Nominal => {
            p.circle_filled(c, r, color);
        }
        Status::Watch => {
            p.circle_stroke(c, r, stroke);
            let arc = (0..=12)
                .map(|i| {
                    let a = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * i as f32 / 12.0;
                    pos2(c.x + r * a.cos(), c.y + r * a.sin())
                })
                .collect();
            p.add(Shape::convex_polygon(arc, color, Stroke::NONE));
        }
        Status::Caution => {
            let pts = vec![
                pos2(c.x, c.y - r),
                pos2(c.x + r, c.y + r),
                pos2(c.x - r, c.y + r),
            ];
            p.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }
        Status::Urgent => {
            let pts = vec![
                pos2(c.x, c.y - r),
                pos2(c.x + r, c.y),
                pos2(c.x, c.y + r),
                pos2(c.x - r, c.y),
            ];
            p.add(Shape::convex_polygon(pts, color, Stroke::NONE));
        }
        Status::Unknown => {
            p.circle_stroke(c, r, stroke);
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                Label::new(
                    RichText::new(
                        "SAFE MODE: rich interface unavailable. Same engine, same results.",
                    )
                    .heading()
                    .strong(),
                )
                .wrap(),
            );
            let (c, _, word) = theme::status_look(self.assessment.overall);
            ui.horizontal(|ui| {
                status_icon(ui, self.assessment.overall, c);
                ui.label(
                    RichText::new(format!("Overall: {word}"))
                        .color(c)
                        .size(20.0),
                );
            });
            ui.separator();
            egui::Grid::new("board")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    for h in ["Indicator", "Value", "Status", "Last measured"] {
                        ui.label(RichText::new(h).color(theme::TEXT_MUTED).strong());
                    }
                    ui.end_row();
                    for i in &self.assessment.indicators {
                        let (c, _, word) = theme::status_look(i.status);
                        ui.label(self.name_of(&i.indicator_id));
                        ui.label(i.value.map_or("-".into(), |v| format!("{v:.1}")));
                        ui.horizontal(|ui| {
                            status_icon(ui, i.status, c);
                            ui.label(RichText::new(word).color(c).strong());
                        });
                        ui.label(i.last_met_s.map_or("-".into(), |s| {
                            format!("{:.1} h ago", (self.now_met_s - s) / 3600.0)
                        }));
                        ui.end_row();
                    }
                });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([760.0, 560.0])
            .with_title("AHC Safe Mode"),
        ..Default::default()
    };
    eframe::run_native(
        "AHC Safe Mode",
        options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}
