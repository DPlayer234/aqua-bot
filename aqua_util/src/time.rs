use std::sync::RwLock;

use serenity::model::Timestamp;
use serenity::utils::Content;
use chrono::prelude::*;

pub const SHORT_TIME: Option<char> = Some('t');
pub const LONG_TIME: Option<char> = Some('T');
pub const SHORT_DATE: Option<char> = Some('d');
pub const LONG_DATE: Option<char> = Some('D');
pub const SHORT_DATE_TIME: Option<char> = Some('f');
pub const LONG_DATE_TIME: Option<char> = Some('F');
pub const RELATIVE: Option<char> = Some('R');

static STARTUP_TIME: RwLock<Option<DateTime<Utc>>> = RwLock::new(None);

pub trait TimestampFormat {
	fn mention(&self, format: Option<char>) -> Content;
}

pub fn mark_startup_time() {
	let mut l = STARTUP_TIME.write().expect("startup_time mutex poisoned");
	*l = Some(Utc::now());
}

pub fn get_startup_time() -> Option<DateTime<Utc>> {
	*STARTUP_TIME.read().expect("startup_time mutex poisoned")
}

impl TimestampFormat for Timestamp {
	fn mention(&self, format: Option<char>) -> Content {
		if let Some(format_raw) = format {
			format!("<t:{}:{}>", self.timestamp(), format_raw).into()
		} else {
			format!("<t:{}>", self.timestamp()).into()
		}
	}
}
