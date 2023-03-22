//! General utilities for use by this bot and maybe other apps.

use serenity::utils::Color;

pub mod time;

/// A general color that can be used for various embeds.
pub const DEFAULT_EMBED_COLOR: Color = Color::new(0x00DD_A0DD);

/// A general color that can be used for embeds indicating errors.
pub const ERROR_EMBED_COLOR: Color = Color::new(0x00CF_0025);

/// Copyright footer text for use in embeds or whatever.
pub const COPYRIGHT_FOOTER: &str = "Aqua Bot © Vamplay#0186 2023";
