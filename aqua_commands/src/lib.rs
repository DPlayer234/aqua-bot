use serenity::framework::StandardFramework;

mod imports;
mod general;
mod hooks;

pub trait StandardFrameworkExt {
	fn configure_aqua_commands(self) -> Self;
}

impl StandardFrameworkExt for StandardFramework {
	fn configure_aqua_commands(self) -> Self {
		self.group(&general::GENERAL_GROUP)
			.after(hooks::after)
			.on_dispatch_error(hooks::dispatch_error)
	}
}
