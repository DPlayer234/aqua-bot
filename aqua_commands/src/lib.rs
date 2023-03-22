//! Provides the commands to be used by the bot.
//! Allows a one-method setup with the [`StandardFramework`].
//! Nothing else is exported.
//! 
//! Generally, each folder here corresponds to a command group.
//! The mod.rs of each group defines the group itself and imports the command methods
//! while each other file defines a single command.

use serenity::framework::StandardFramework;

mod general;
mod internal;

use internal::hooks;

/// An extension for [`StandardFramework`] to allow the setup.
pub trait StandardFrameworkExt {
	/// Configures the `aqua_commands` on this framework instance.
	#[must_use]
	fn configure_aqua_commands(self) -> Self;
}

impl StandardFrameworkExt for StandardFramework {
	fn configure_aqua_commands(self) -> Self {
		self.group(&general::GENERAL_GROUP)
			.after(hooks::after)
			.on_dispatch_error(hooks::dispatch_error)
	}
}
