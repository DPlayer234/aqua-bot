use std::env;

use aqua_commands::StandardFrameworkExt;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}

	async fn resume(&self, _: Context, _: ResumedEvent) {
		println!("Resumed connection.");
	}

	async fn guild_create(&self, _: Context, guild: Guild, is_new: bool) {
		if is_new {
			println!("Joined Guild: {:?} [{}]", guild.name, guild.id);
		} else {
			println!("Is in Guild: {:?} [{}]", guild.name, guild.id);
		}
	}
}

#[tokio::main]
async fn main() {
	let token = env::var("BOT_TOKEN").expect("BOT_TOKEN env var must be set.");
	let intents
		= GatewayIntents::GUILDS // Needed for a complete cache
		| GatewayIntents::GUILD_MESSAGES // Needed because this bot uses message commands
		| GatewayIntents::DIRECT_MESSAGES
		| GatewayIntents::MESSAGE_CONTENT;

	let framework = serenity::framework::StandardFramework::new()
		.configure(|c| c
			.prefix("++")
			.delimiter(' '))
		.configure_aqua_commands();

	let mut builder = Client::builder(token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Error creating client.");

	aqua_util::time::mark_startup_time();

	if let Err(reason) = builder.start().await {
		println!("Client Error: {:?}", reason);
	}
}
