use crate::internal::prelude::*;
use aqua_util::time::get_startup_time;
use chrono::prelude::*;

#[command]
#[num_args(0)]
pub async fn hi(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
	let now = Utc::now();
	let startup_time = get_startup_time().expect("startup time was not marked");
	let uptime = now - startup_time;
	let current_user = ctx.cache.current_user();

	msg.channel_id.send_message(&ctx, |m| m
		.add_embed(|e| e
			.title(format!("Info: {}#{:04}", current_user.name, current_user.discriminator))
			.description(MessageBuilder::new()
				.push_bold("Uptime:")
				.push(' ')
				.push_line(
					format!(
						"{}.{:02}:{:02}:{:02}.{:03}",
						uptime.num_days(),
						uptime.num_hours() % 24,
						uptime.num_minutes() % 60,
						uptime.num_seconds() % 60,
						uptime.num_milliseconds() % 1000
					)
				)
				
				.push_bold("Time:")
				.push(' ')
				.push(now.format("%a, %d/%m/%Y %H:%M:%S"))
				.push_line(" UTC+0")
				
				.push_bold("Shard:")
				.push(" ID ")
				.push_line(ctx.shard_id)

				.push_bold("Guilds:")
				.push(' ')
				.push_line(ctx.cache.guild_count())
			)
			.footer(|f| f
				.text(aqua_util::COPYRIGHT_FOOTER)
			)
			.thumbnail(current_user.face())
			.color(aqua_util::DEFAULT_EMBED_COLOR)
		)
	).await?;
	Ok(())
}
