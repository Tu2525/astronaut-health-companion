//! Mirror of `design/tokens.css`, same names (`--status-nominal` is `STATUS_NOMINAL`).
//! This is the only file in `apps/egui` allowed to hold colour literals.
//! When `tokens.css` changes, update the values here (see `design/README.md`, section 2).
#![allow(dead_code)] // the whole token set is mirrored, not only what is used today

use ahc_core::{SourceStatus, Status};
use eframe::egui::{Color32, Context, Stroke, TextStyle, Visuals};

// Status: meaning is fixed. Always shown with icon + word (docs/contract.md section 2).
pub const STATUS_NOMINAL: Color32 = Color32::from_rgb(0x3e, 0xcf, 0x8e);
pub const STATUS_WATCH: Color32 = Color32::from_rgb(0x4c, 0x9a, 0xff);
pub const STATUS_CAUTION: Color32 = Color32::from_rgb(0xf5, 0xa5, 0x24);
pub const STATUS_URGENT: Color32 = Color32::from_rgb(0xf0, 0x44, 0x5a);
pub const STATUS_UNKNOWN: Color32 = Color32::from_rgb(0x8a, 0x93, 0xa6);

// Surfaces
pub const BG: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);
pub const SURFACE: Color32 = Color32::from_rgb(0x05, 0x06, 0x07);
pub const SURFACE_RAISED: Color32 = Color32::from_rgb(0x0d, 0x0f, 0x12);
/// Flag rows, ladder steps, action tiles.
pub const SURFACE_SUNKEN: Color32 = Color32::from_rgb(0x0a, 0x0b, 0x0d);
/// Hover fill, subtle row dividers.
pub const SURFACE_HOVER: Color32 = Color32::from_rgb(0x15, 0x18, 0x1c);
/// Top glow behind the web body stage (no egui use).
pub const STAGE_GLOW: Color32 = Color32::from_rgb(0x14, 0x17, 0x1b);
pub const BORDER: Color32 = Color32::from_rgb(0x23, 0x26, 0x2b);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0x45, 0x4a, 0x52);

// Text
pub const TEXT: Color32 = Color32::from_rgb(0xf2, 0xf3, 0xf5);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x8b, 0x90, 0x97);
pub const TEXT_ON_STATUS: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);

// Brand / interactive
pub const ACCENT: Color32 = Color32::from_rgb(0xf2, 0xf3, 0xf5);
pub const ACCENT_HOVER: Color32 = Color32::from_rgb(0xd6, 0xd8, 0xdc);
pub const FOCUS_RING: Color32 = Color32::from_rgb(0xf2, 0xf3, 0xf5);

/// 3D body: nominal organs stay near-silver (web only; mirrored for completeness).
pub const ORGAN_NEUTRAL: Color32 = Color32::from_rgb(0x9a, 0xa3, 0xad);

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
pub const RADIUS_SM: f32 = 2.0;
pub const RADIUS_MD: f32 = 4.0;
pub const RADIUS_LG: f32 = 6.0;
pub const RADIUS_PILL: f32 = 999.0;
pub const SPACE_1: f32 = 4.0;
pub const SPACE_2: f32 = 8.0;
pub const SPACE_3: f32 = 12.0;
pub const SPACE_4: f32 = 16.0;
pub const SPACE_6: f32 = 24.0;
pub const SPACE_8: f32 = 32.0;
/// Gloves, motion, low light.
pub const TOUCH_TARGET: f32 = 48.0;
// `--font-display`, `--font-ui` and `--font-mono` are font stacks; egui's built-in
// fonts are used for now (the web self-hosts its display font).

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

/// Chip colour for a range's source status (`--source-*` tokens).
pub fn source_colour(s: SourceStatus) -> Color32 {
    match s {
        SourceStatus::Sourced => SOURCE_SOURCED,
        SourceStatus::Adapted => SOURCE_ADAPTED,
        SourceStatus::Placeholder => SOURCE_PLACEHOLDER,
    }
}

/// Dark high-contrast look from the tokens. Call once at start-up.
pub fn apply(ctx: &Context) {
    // Larger than egui's defaults: read at arm's length, in motion and low light.
    ctx.style_mut(|s| {
        for (style, size) in [
            (TextStyle::Small, 13.0_f32),
            (TextStyle::Body, 16.0),
            (TextStyle::Button, 16.0),
            (TextStyle::Monospace, 15.0),
            (TextStyle::Heading, 22.0),
        ] {
            if let Some(font) = s.text_styles.get_mut(&style) {
                font.size = size;
            }
        }
        s.spacing.item_spacing.y = SPACE_2;
    });
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `--token` in design/tokens.css, by name, with the mirrored value.
    fn mirror() -> Vec<(&'static str, String)> {
        let hex = |c: Color32| format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b());
        let px = |v: f32| format!("{v}px");
        let mut m: Vec<(&'static str, String)> = [
            ("status-nominal", STATUS_NOMINAL),
            ("status-watch", STATUS_WATCH),
            ("status-caution", STATUS_CAUTION),
            ("status-urgent", STATUS_URGENT),
            ("status-unknown", STATUS_UNKNOWN),
            ("bg", BG),
            ("surface", SURFACE),
            ("surface-raised", SURFACE_RAISED),
            ("surface-sunken", SURFACE_SUNKEN),
            ("surface-hover", SURFACE_HOVER),
            ("stage-glow", STAGE_GLOW),
            ("border", BORDER),
            ("border-strong", BORDER_STRONG),
            ("text", TEXT),
            ("text-muted", TEXT_MUTED),
            ("text-on-status", TEXT_ON_STATUS),
            ("accent", ACCENT),
            ("accent-hover", ACCENT_HOVER),
            ("focus-ring", FOCUS_RING),
            ("organ-neutral", ORGAN_NEUTRAL),
            ("source-sourced", SOURCE_SOURCED),
            ("source-adapted", SOURCE_ADAPTED),
            ("source-placeholder", SOURCE_PLACEHOLDER),
            ("hazard-radiation", HAZARD_RADIATION),
            ("hazard-isolation", HAZARD_ISOLATION),
            ("hazard-distance", HAZARD_DISTANCE),
            ("hazard-gravity", HAZARD_GRAVITY),
            ("hazard-closed-environment", HAZARD_CLOSED_ENVIRONMENT),
        ]
        .into_iter()
        .map(|(n, c)| (n, hex(c)))
        .collect();
        m.extend(
            [
                ("radius-sm", RADIUS_SM),
                ("radius-md", RADIUS_MD),
                ("radius-lg", RADIUS_LG),
                ("radius-pill", RADIUS_PILL),
                ("space-1", SPACE_1),
                ("space-2", SPACE_2),
                ("space-3", SPACE_3),
                ("space-4", SPACE_4),
                ("space-6", SPACE_6),
                ("space-8", SPACE_8),
                ("touch-target", TOUCH_TARGET),
            ]
            .into_iter()
            .map(|(n, v)| (n, px(v))),
        );
        m
    }

    /// Fails when tokens.css gains a token or changes a value that theme.rs does not mirror.
    #[test]
    fn theme_mirrors_design_tokens() {
        let css = include_str!("../../../design/tokens.css");
        let mirror = mirror();
        for line in css.lines() {
            let Some(rest) = line.trim().strip_prefix("--") else {
                continue;
            };
            let Some((name, value)) = rest.split_once(':') else {
                continue;
            };
            let value = value.split(';').next().unwrap_or("").trim().to_lowercase();
            if name.starts_with("font-") {
                continue; // font stacks: egui's built-in fonts are used (see above)
            }
            let ours = mirror.iter().find(|(n, _)| *n == name);
            assert_eq!(
                ours.map(|(_, v)| v.as_str()),
                Some(value.as_str()),
                "--{name}"
            );
        }
    }

    #[test]
    fn statuses_have_distinct_icons_and_words() {
        let all = [
            Status::Unknown,
            Status::Nominal,
            Status::Watch,
            Status::Caution,
            Status::Urgent,
        ];
        let looks: Vec<_> = all.iter().map(|s| status_look(*s)).collect();
        for (i, a) in looks.iter().enumerate() {
            for b in &looks[i + 1..] {
                assert_ne!(a.1, b.1, "icon");
                assert_ne!(a.2, b.2, "word");
            }
        }
    }
}
