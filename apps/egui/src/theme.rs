//! Mirror of `design/tokens.css`, same names (`--status-nominal` is `STATUS_NOMINAL`).
//! This is the only file in `apps/egui` allowed to hold colour literals.
//! When `tokens.css` changes, update the values here (see `design/README.md`, section 2).
#![allow(dead_code)] // the whole token set is mirrored, not only what is used today

use ahc_core::Status;
use eframe::egui::{Color32, Context, Stroke, Visuals};

// Status: meaning is fixed. Always shown with icon + word (docs/contract.md section 2).
pub const STATUS_NOMINAL: Color32 = Color32::from_rgb(0x3e, 0xcf, 0x8e);
pub const STATUS_WATCH: Color32 = Color32::from_rgb(0x4c, 0x9a, 0xff);
pub const STATUS_CAUTION: Color32 = Color32::from_rgb(0xf5, 0xa5, 0x24);
pub const STATUS_URGENT: Color32 = Color32::from_rgb(0xf0, 0x44, 0x5a);
pub const STATUS_UNKNOWN: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa6);

// Surfaces
pub const BG: Color32 = Color32::from_rgb(0x0b, 0x0f, 0x17);
pub const SURFACE: Color32 = Color32::from_rgb(0x12, 0x18, 0x26);
pub const SURFACE_RAISED: Color32 = Color32::from_rgb(0x1a, 0x22, 0x33);
pub const BORDER: Color32 = Color32::from_rgb(0x26, 0x30, 0x46);

// Text
pub const TEXT: Color32 = Color32::from_rgb(0xe6, 0xea, 0xf2);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa6);
pub const TEXT_ON_STATUS: Color32 = Color32::from_rgb(0x0b, 0x0f, 0x17);

// Brand / interactive
pub const ACCENT: Color32 = Color32::from_rgb(0x7c, 0x9c, 0xff);
pub const FOCUS_RING: Color32 = Color32::from_rgb(0xa9, 0xbc, 0xff);

// Data honesty chips
pub const SOURCE_SOURCED: Color32 = Color32::from_rgb(0x3e, 0xcf, 0x8e);
pub const SOURCE_ADAPTED: Color32 = Color32::from_rgb(0xf5, 0xa5, 0x24);
pub const SOURCE_PLACEHOLDER: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa6);

// Hazard lens (decorative)
pub const HAZARD_RADIATION: Color32 = Color32::from_rgb(0xc0, 0x84, 0xfc);
pub const HAZARD_ISOLATION: Color32 = Color32::from_rgb(0x67, 0xe8, 0xf9);
pub const HAZARD_DISTANCE: Color32 = Color32::from_rgb(0xf0, 0xab, 0xfc);
pub const HAZARD_GRAVITY: Color32 = Color32::from_rgb(0xa3, 0xe6, 0x35);
pub const HAZARD_CLOSED_ENVIRONMENT: Color32 = Color32::from_rgb(0xfd, 0xa4, 0xaf);

// Shape, space, type (points; the CSS px values map 1:1)
pub const RADIUS_SM: f32 = 6.0;
pub const RADIUS_MD: f32 = 12.0;
pub const RADIUS_LG: f32 = 20.0;
pub const SPACE_1: f32 = 4.0;
pub const SPACE_2: f32 = 8.0;
pub const SPACE_3: f32 = 12.0;
pub const SPACE_4: f32 = 16.0;
pub const SPACE_6: f32 = 24.0;
pub const SPACE_8: f32 = 32.0;
/// Gloves, motion, low light.
pub const TOUCH_TARGET: f32 = 48.0;
// `--font-ui` and `--font-mono` are font stacks; egui's built-in fonts are used for now.

/// Colour, icon glyph and word for a status (docs/contract.md section 2).
/// The board draws the icon as a shape; the glyph is kept for text-only uses.
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
pub fn apply(ctx: &Context) {
    let mut v = Visuals::dark();
    v.panel_fill = BG;
    v.window_fill = SURFACE;
    v.extreme_bg_color = BG;
    v.faint_bg_color = SURFACE;
    v.override_text_color = Some(TEXT);
    v.hyperlink_color = ACCENT;
    // Keyboard-first: egui draws the keyboard focus ring with `selection.stroke`.
    v.selection.stroke = Stroke::new(2.0_f32, FOCUS_RING);
    v.selection.bg_fill = SURFACE_RAISED;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, BORDER);
    ctx.set_visuals(v);
}
