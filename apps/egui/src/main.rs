//! ahc-safe: Safe-Mode Console. B1 status board (SAFE-2) over the shared event log
//! (SAFE-3), built natively and as WASM for `/safe/`.
//! No status logic here: everything comes from `ahc_core::assess`, shaped for
//! display by `board.rs`.
mod board;
mod entry;
mod storage;
mod theme;

use ahc_core::{assess, demo, registry, AssessInput, Event, Indicator, KitItem, Status};
use board::{Board, FlagCard, LADDER};
use eframe::egui::{
    self, pos2, Color32, CornerRadius, Frame, Label, Margin, RichText, Sense, Shape, Stroke, Vec2,
};
use storage::{Changes, Settings};

/// How often the board is re-assessed while nothing changes, so staleness and
/// the clock move on.
const TICK_S: f64 = 10.0;

struct App {
    events: Vec<Event>,
    settings: Settings,
    kit: Vec<KitItem>,
    registry: Vec<Indicator>,
    log_location: String,
    notice: Option<String>,
    changes: Changes,
    was_focused: bool,
    mission_start_ms: f64,
    assessed_at: f64,
    board: Board,
    draft: entry::Draft,
    /// Result of the last quick entry: Ok(confirmation) or Err(why it was not logged).
    entry_result: Option<Result<String, String>>,
    /// Put the keyboard in the value field on the next frame (keyboard-first).
    focus_entry: bool,
    /// Whether the value field had focus at the end of the last frame. egui drops a
    /// text field's focus on Esc before this frame's code runs, so Esc is matched
    /// against this instead of the current focus.
    entry_had_focus: bool,
}

/// A key press that is not an auto-repeat (holding Enter must not submit twice).
fn pressed_once(i: &egui::InputState, key: egui::Key) -> bool {
    i.events.iter().any(|e| {
        matches!(e, egui::Event::Key { key: k, pressed: true, repeat: false, modifiers, .. }
            if *k == key && modifiers.is_none())
    })
}

/// Fixed id so the value field can be focused and checked before it is drawn.
const ENTRY_FIELD: &str = "quick_entry_value";

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply(&cc.egui_ctx);
        let loaded = storage::load_events();
        let settings = storage::load_settings();
        let kit = demo::kit();
        let registry = registry();
        let mission_start_ms = storage::mission_start_ms();
        let events = loaded.events.unwrap_or_default();
        let board = Self::assess_board(&events, &settings, &kit, &registry, mission_start_ms);
        Self {
            events,
            settings,
            kit,
            registry,
            log_location: storage::location(),
            notice: loaded.notice,
            changes: Changes::watch(&cc.egui_ctx),
            was_focused: true,
            mission_start_ms,
            assessed_at: 0.0,
            board,
            draft: entry::Draft::default(),
            entry_result: None,
            focus_entry: true,
            entry_had_focus: false,
        }
    }

    /// Logs the drafted reading: one `reading` event appended to the shared log,
    /// then the board is re-assessed at once so the crew sees the engine's verdict.
    fn submit_entry(&mut self) {
        let Some(indicator) = self.registry.get(self.draft.selected) else {
            return;
        };
        let value = match entry::parse_value(&self.draft.text) {
            Ok(v) => v,
            Err(e) => {
                self.entry_result = Some(Err(e));
                return;
            }
        };
        let now_met_s = storage::now_met_s(
            self.mission_start_ms,
            storage::wall_ms(),
            &self.settings,
            &self.events,
        );
        let event = entry::reading_event(
            entry::new_event_id(),
            indicator,
            value,
            &self.settings,
            now_met_s,
        );
        self.entry_result = Some(match storage::append_event(event) {
            Ok(events) => {
                let logged = format!(
                    "Logged {} {} at MET {}.",
                    indicator.name,
                    board::format_value(value, &indicator.unit),
                    board::format_met(now_met_s)
                );
                self.events = events;
                self.notice = None;
                self.draft.text.clear();
                self.board = Self::assess_board(
                    &self.events,
                    &self.settings,
                    &self.kit,
                    &self.registry,
                    self.mission_start_ms,
                );
                Ok(logged)
            }
            Err(e) => Err(e),
        });
    }

    /// B2 Quick entry. Keyboard: Up/Down choose the indicator, Enter logs,
    /// Esc clears, `/` jumps here from anywhere.
    fn quick_entry(&mut self, ui: &mut egui::Ui) {
        let len = self.registry.len();
        if len == 0 {
            ui.label("Quick entry unavailable: the indicator registry is empty.");
            return;
        }
        let field = egui::Id::new(ENTRY_FIELD);
        let ctx = ui.ctx().clone();
        let focused = ctx.memory(|m| m.has_focus(field));
        // Keys are taken before the text field sees them.
        let (up, down, esc, slash) = ctx.input_mut(|i| {
            let none = egui::Modifiers::NONE;
            let up = focused && i.consume_key(none, egui::Key::ArrowUp);
            let down = focused && i.consume_key(none, egui::Key::ArrowDown);
            let esc = self.entry_had_focus && pressed_once(i, egui::Key::Escape);
            // The field is the only text input, so "/" never means text elsewhere.
            let slash = !focused && i.consume_key(none, egui::Key::Slash);
            if slash {
                // The typed "/" must not land in the field it focuses.
                i.events
                    .retain(|e| !matches!(e, egui::Event::Text(t) if t == "/"));
            }
            (up, down, esc, slash)
        });
        if up {
            self.draft.step(-1, len);
        }
        if down {
            self.draft.step(1, len);
        }
        if esc {
            self.draft.text.clear();
            self.entry_result = None;
        }

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Quick entry").heading().strong());
            ui.add_space(theme::SPACE_2);
            ui.label(
                RichText::new("Up/Down: indicator · Enter: log · Esc: clear · /: jump here")
                    .color(theme::TEXT_MUTED)
                    .small(),
            );
        });
        let mut submit = false;
        ui.horizontal_wrapped(|ui| {
            self.draft.selected = self.draft.selected.min(len - 1);
            let picked_before = self.draft.selected;
            let selected = &self.registry[picked_before];
            egui::ComboBox::from_id_salt("quick_entry_indicator")
                .width(240.0)
                .selected_text(&selected.name)
                .show_ui(ui, |ui| {
                    for (i, ind) in self.registry.iter().enumerate() {
                        ui.selectable_value(&mut self.draft.selected, i, &ind.name);
                    }
                });
            let unit = selected.unit.clone();
            let resp = ui.add(
                egui::TextEdit::singleline(&mut self.draft.text)
                    .id(field)
                    .desired_width(120.0)
                    .hint_text("value"),
            );
            ui.label(RichText::new(unit).strong());
            // Enter makes a single-line field give up focus; take it back either way.
            let enter_left = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if enter_left && ui.input(|i| pressed_once(i, egui::Key::Enter)) {
                submit = true;
            }
            if ui.button(RichText::new("Log reading").strong()).clicked() {
                submit = true;
            }
            // Picking an indicator with the mouse returns the keyboard to the field.
            let picked = self.draft.selected != picked_before;
            if self.focus_entry || slash || submit || esc || enter_left || picked {
                resp.request_focus();
                self.focus_entry = false;
            }
            self.entry_had_focus = resp.has_focus() || resp.lost_focus();
        });
        if submit {
            self.submit_entry();
        }
        match &self.entry_result {
            Some(Ok(msg)) => {
                ui.label(RichText::new(msg).strong());
            }
            Some(Err(msg)) => {
                ui.add(Label::new(RichText::new(msg).strong()).wrap());
            }
            None => {
                ui.label(
                    RichText::new(
                        "Value in the unit shown. The engine judges it; nothing is edited or deleted.",
                    )
                    .color(theme::TEXT_MUTED)
                    .small(),
                );
            }
        }
    }

    fn assess_board(
        events: &[Event],
        settings: &Settings,
        kit: &[KitItem],
        registry: &[Indicator],
        mission_start_ms: f64,
    ) -> Board {
        let now_met_s = storage::now_met_s(mission_start_ms, storage::wall_ms(), settings, events);
        let input = AssessInput {
            crew_id: settings.crew_id.clone(),
            events: events.to_vec(),
            now_met_s,
            kit: kit.to_vec(),
            registry: None,
        };
        Board::build(
            &assess(&input),
            registry,
            &settings.crew_id,
            events,
            now_met_s,
            settings.one_way_delay_s,
        )
    }

    /// Re-read storage when another tab wrote to it or the window regains focus
    /// (`apps/egui/AGENTS.md`), and re-assess on a timer.
    fn refresh(&mut self, ctx: &egui::Context) {
        let (focused, time) = ctx.input(|i| (i.focused, i.time));
        let regained_focus = focused && !self.was_focused;
        self.was_focused = focused;
        let reload = self.changes.take() || regained_focus;
        if reload {
            let loaded = storage::load_events();
            // An unreadable re-read keeps the last good log on screen, with the notice.
            if let Some(events) = loaded.events {
                self.events = events;
            }
            self.notice = loaded.notice;
            self.settings = storage::load_settings();
            // Keep the current start if none is stored (or it cannot be read), so the
            // clock never jumps back.
            if let Some(ms) = storage::stored_mission_start_ms() {
                self.mission_start_ms = ms;
            }
        }
        if reload || time - self.assessed_at >= TICK_S {
            self.board = Self::assess_board(
                &self.events,
                &self.settings,
                &self.kit,
                &self.registry,
                self.mission_start_ms,
            );
            self.assessed_at = time;
        }
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(TICK_S));
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
        self.refresh(ctx);
        // Drawn before the board so its result shows in the same frame.
        egui::TopBottomPanel::bottom("quick_entry")
            .frame(
                Frame::new()
                    .fill(theme::SURFACE_RAISED)
                    .stroke(Stroke::new(1.0_f32, theme::BORDER))
                    .inner_margin(Margin::symmetric(
                        theme::SPACE_4 as i8,
                        theme::SPACE_3 as i8,
                    )),
            )
            .show(ctx, |ui| self.quick_entry(ui));
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
                ui.add(
                    Label::new(
                        RichText::new(format!(
                            "Log: {} events · {}",
                            self.events.len(),
                            self.log_location
                        ))
                        .color(theme::TEXT_MUTED)
                        .small(),
                    )
                    .wrap(),
                );
                if let Some(notice) = &self.notice {
                    ui.add(Label::new(RichText::new(notice).strong()).wrap());
                }
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

#[cfg(not(target_arch = "wasm32"))]
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
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

/// WASM entry: draws into `#safe_canvas` from `index.html` and replaces the
/// `#safe_status` loading text, or shows the error there.
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;
    let document = web_sys::window().and_then(|w| w.document());
    let status = document
        .as_ref()
        .and_then(|d| d.get_element_by_id("safe_status"));
    let canvas = document
        .as_ref()
        .and_then(|d| d.get_element_by_id("safe_canvas"))
        .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok());
    let Some(canvas) = canvas else {
        if let Some(s) = &status {
            s.set_text_content(Some(
                "Safe mode could not start: no #safe_canvas on the page.",
            ));
        }
        return;
    };
    wasm_bindgen_futures::spawn_local(async move {
        let started = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(App::new(cc)))),
            )
            .await;
        if let Some(s) = status {
            match started {
                Ok(()) => s.remove(),
                Err(e) => s.set_text_content(Some(&format!("Safe mode could not start: {e:?}"))),
            }
        }
    });
}
