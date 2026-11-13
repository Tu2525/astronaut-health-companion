//! Mirror of `design/tokens.css` (same names). The only file allowed to hold colour literals.
use ahc_core::Status;
use eframe::egui::Color32;

pub const STATUS_NOMINAL: Color32 = Color32::from_rgb(0x3e, 0xcf, 0x8e);
pub const STATUS_WATCH: Color32 = Color32::from_rgb(0x4c, 0x9a, 0xff);
pub const STATUS_CAUTION: Color32 = Color32::from_rgb(0xf5, 0xa5, 0x24);
pub const STATUS_URGENT: Color32 = Color32::from_rgb(0xf0, 0x44, 0x5a);
pub const STATUS_UNKNOWN: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa6);

pub const BG: Color32 = Color32::from_rgb(0x0b, 0x0f, 0x17);
pub const SURFACE: Color32 = Color32::from_rgb(0x12, 0x18, 0x26);
pub const BORDER: Color32 = Color32::from_rgb(0x26, 0x30, 0x46);
pub const TEXT: Color32 = Color32::from_rgb(0xe6, 0xea, 0xf2);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa6);

/// Colour, icon and word for a status (docs/contract.md section 2).
pub fn status_look(s: Status) -> (Color32, &'static str, &'static str) {
    match s {
        Status::Nominal => (STATUS_NOMINAL, "●", "Nominal"),
        Status::Watch => (STATUS_WATCH, "◐", "Watch"),
        Status::Caution => (STATUS_CAUTION, "▲", "Caution"),
        Status::Urgent => (STATUS_URGENT, "◆", "Urgent"),
        Status::Unknown => (STATUS_UNKNOWN, "○", "Unknown"),
    }
}

/// Dark high-contrast look from the tokens. Call once at start-up.
pub fn apply(ctx: &eframe::egui::Context) {
    let mut v = eframe::egui::Visuals::dark();
    v.panel_fill = BG;
    v.window_fill = SURFACE;
    v.extreme_bg_color = BG;
    v.faint_bg_color = SURFACE;
    v.override_text_color = Some(TEXT);
    v.widgets.noninteractive.bg_stroke = eframe::egui::Stroke::new(1.0_f32, BORDER);
    ctx.set_visuals(v);
}
