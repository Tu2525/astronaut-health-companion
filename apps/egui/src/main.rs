//! ahc-safe: Safe-Mode Console. B1 status board (SAFE-2).
//! No status logic here: everything comes from `ahc_core::assess`, shaped for
//! display by `board.rs`.
mod board;
mod theme;

use ahc_core::{assess, demo, registry, AssessInput, Event, Indicator, KitItem, Status};
use board::{Board, FlagCard, LADDER};
use eframe::egui::{
    self, pos2, Color32, CornerRadius, Frame, Label, Margin, RichText, Sense, Shape, Stroke, Vec2,
};

struct App {
    events: Vec<Event>,
    kit: Vec<KitItem>,
    registry: Vec<Indicator>,
    now_met_s: f64,
    board: Board,
}

impl App {
    fn new() -> Self {
        let events = demo::events();
        let kit = demo::kit();
        let registry = registry();
        let now_met_s = demo::NOW_MET_S;
        let board = Self::assess_board(&events, &kit, &registry, now_met_s);
        Self {
            events,
            kit,
            registry,
            now_met_s,
            board,
        }
    }

    fn assess_board(
        events: &[Event],
        kit: &[KitItem],
        registry: &[Indicator],
        now_met_s: f64,
    ) -> Board {
        let input = AssessInput {
            crew_id: demo::CREW_ID.into(),
            events: events.to_vec(),
            now_met_s,
            kit: kit.to_vec(),
            registry: None,
        };
        Board::build(
            &assess(&input),
            registry,
            demo::CREW_ID,
            events,
            now_met_s,
            board::DEMO_ONE_WAY_DELAY_S,
        )
    }

    /// Re-run the engine after the log or the clock changes (SAFE-3 and SAFE-4 call this).
    #[allow(dead_code)]
    fn reassess(&mut self) {
        self.board = Self::assess_board(&self.events, &self.kit, &self.registry, self.now_met_s);
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

/// Colour + icon + word, the only way a status is ever shown.
fn status_badge(ui: &mut egui::Ui, s: Status, size: f32) {
    let (c, _, word) = theme::status_look(s);
    ui.horizontal(|ui| {
        status_icon(ui, s, c);
        ui.label(RichText::new(word).color(c).strong().size(size));
    });
}

fn card() -> Frame {
    Frame::new()
        .fill(theme::SURFACE)
        .stroke(Stroke::new(1.0_f32, theme::BORDER))
        .corner_radius(CornerRadius::same(theme::RADIUS_MD as u8))
        .inner_margin(Margin::same(theme::SPACE_4 as i8))
}

/// The four ladder steps; the current one is bracketed and bold, not only coloured.
fn ladder(ui: &mut egui::Ui, step: usize, colour: Color32) {
    ui.horizontal_wrapped(|ui| {
        for (i, (_, label)) in LADDER.iter().enumerate() {
            let text = format!("{} {label}", i + 1);
            if i == step {
                ui.label(RichText::new(format!("[ {text} ]")).color(colour).strong());
            } else {
                ui.label(RichText::new(text).color(theme::TEXT_MUTED));
            }
            if i + 1 < LADDER.len() {
                ui.label(RichText::new(">").color(theme::TEXT_MUTED));
            }
        }
    });
}

fn flag_card(ui: &mut egui::Ui, f: &FlagCard) {
    let (c, _, _) = theme::status_look(f.status);
    card().show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            status_badge(ui, f.status, 16.0);
            ui.label(RichText::new(&f.name).strong());
        });
        ui.add(Label::new(&f.summary).wrap());
        ui.add(Label::new(RichText::new(format!("Next step: {}", f.next_step)).strong()).wrap());
        ui.horizontal(|ui| {
            ui.label(RichText::new("Escalation:").color(theme::TEXT_MUTED));
            ladder(ui, f.step, c);
        });
    });
}

fn source_chip(ui: &mut egui::Ui, s: ahc_core::SourceStatus) {
    let c = theme::source_colour(s);
    Frame::new()
        .stroke(Stroke::new(1.0_f32, c))
        .corner_radius(CornerRadius::same(theme::RADIUS_SM as u8))
        .inner_margin(Margin::symmetric(theme::SPACE_2 as i8, 0))
        .show(ui, |ui| {
            ui.label(RichText::new(board::source_word(s)).color(c).small());
        });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let b = &self.board;
        egui::TopBottomPanel::top("banner")
            .frame(
                Frame::new()
                    .fill(theme::SURFACE_RAISED)
                    .stroke(Stroke::new(2.0_f32, theme::ACCENT))
                    .inner_margin(Margin::symmetric(
                        theme::SPACE_4 as i8,
                        theme::SPACE_3 as i8,
                    )),
            )
            .show(ctx, |ui| {
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
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!(
                        "Mission day {} · MET {} · [{}] · Comms: {}",
                        b.mission_day, b.met, b.environment, b.comms
                    ));
                });
                ui.horizontal(|ui| {
                    status_badge(ui, b.overall, 20.0);
                    ui.label(RichText::new("overall").color(theme::TEXT_MUTED));
                    ui.add_space(theme::SPACE_4);
                    ui.label(RichText::new("Escalation:").color(theme::TEXT_MUTED));
                    ui.label(RichText::new(LADDER[b.escalation_step].1).strong());
                });
            });

        egui::CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(theme::BG)
                    .inner_margin(Margin::same(theme::SPACE_4 as i8)),
            )
            .show(ctx, |ui| {
                // Both directions: the table is wider than the minimum window width.
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.heading("Active flags");
                    if b.flags.is_empty() {
                        ui.label(RichText::new("No active flags.").color(theme::TEXT_MUTED));
                    }
                    for f in &b.flags {
                        flag_card(ui, f);
                    }
                    if b.more_flags > 0 {
                        ui.label(
                            RichText::new(format!(
                                "+{} more at Watch or worse in the table below",
                                b.more_flags
                            ))
                            .color(theme::TEXT_MUTED),
                        );
                    }

                    ui.add_space(theme::SPACE_4);
                    ui.heading("Indicators");
                    egui::Grid::new("board")
                        .striped(true)
                        .num_columns(6)
                        .spacing([theme::SPACE_4, theme::SPACE_2])
                        .max_col_width(300.0)
                        .show(ui, |ui| {
                            for h in [
                                "Indicator",
                                "Value",
                                "Status",
                                "Why",
                                "Source",
                                "Last measured",
                            ] {
                                ui.label(RichText::new(h).color(theme::TEXT_MUTED).strong());
                            }
                            ui.end_row();
                            for r in &b.rows {
                                ui.label(&r.name);
                                ui.label(&r.value);
                                status_badge(ui, r.status, 16.0);
                                ui.add(Label::new(&r.reason).wrap());
                                source_chip(ui, r.source);
                                ui.label(&r.last_measured);
                                ui.end_row();
                            }
                        });
                });
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // Fits a small laptop screen at 150 % scaling; the board scrolls.
            .with_inner_size([1080.0, 500.0])
            .with_min_inner_size([640.0, 400.0])
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
