use crate::internal::prelude::*;
use serenity::framework::standard::ArgError;
use aqua_util::time::{TimestampMention, SHORT_DATE_TIME};
use aqua_util::ERROR_EMBED_COLOR;

#[command]
#[max_args(1)]
pub async fn what(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	match args.single::<ChannelId>() {
		Ok(id) => what_some(ctx, msg, id).await,
		Err(ArgError::Eos) => what_here(ctx, msg).await,
		Err(err) => Err(Box::new(err)) 
	}
}

async fn what_here(ctx: &Context, msg: &Message) -> CommandResult {
	what_some(ctx, msg, msg.channel_id).await
}

async fn what_some(ctx: &Context, msg: &Message, id: ChannelId) -> CommandResult {
	if let Ok(channel) = id.to_channel(&ctx).await {
		what_channel(ctx, msg, channel).await
	} else {
		what_other(ctx, msg, id).await
	}
}

async fn what_channel(ctx: &Context, msg: &Message, channel: Channel) -> CommandResult {
	let guild = match channel {
		Channel::Guild(ref guild_channel) => guild_channel.guild(ctx),
		Channel::Category(ref category_channel) => category_channel.guild_id.to_guild_cached(ctx),
		_ => None,
	};

	msg.channel_id.send_message(&ctx, |m| m
		.add_embed(|e| channel_info_embed(&channel, guild.as_ref(), e))).await?;
	Ok(())
}

async fn what_other(ctx: &Context, msg: &Message, id: ChannelId) -> CommandResult {
	msg.channel_id.send_message(&ctx, |m| m
		.add_embed(|e| other_info_embed(id, e))).await?;
	Ok(())
}

fn channel_info_embed<'a>(channel: &Channel, guild: Option<&Guild>, embed: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
	if let Some(guild) = guild {
		set_guild_info(guild, embed);
	}

	set_channel_info(channel, embed);
	embed.color(DEFAULT_EMBED_COLOR)
}

fn set_guild_info(guild: &Guild, embed: &mut CreateEmbed) {
	let mut builder = MessageBuilder::new();

	builder.push_bold("Snowflake:")
		.push(' ')
		.push_mono_line(guild.id.as_u64());
		
	builder.push_bold("Created At:")
		.push(' ')
		.push_line(guild.id.created_at().mention(SHORT_DATE_TIME));

	if guild.large {
		builder.push_bold_line("Is Large");
	}

	builder.push_bold("Members:")
		.push(' ')
		.push_line(guild.member_count);

	builder.push_bold("Channels:")
		.push(' ')
		.push_line(guild.channels.len());

	builder.push_bold("Roles:")
		.push(' ')
		.push_line(guild.roles.len());

	if guild.premium_subscription_count != 0 {
		builder.push_bold("Nitro Tier:")
			.push(' ')
			.push(guild.premium_tier.num())
			.push(" (Boost Count: ")
			.push(guild.premium_subscription_count)
			.push_line(")");
	}

	if guild.nsfw_level != NsfwLevel::Default {
		builder.push_bold("NSFW Level:")
			.push(' ')
			.push_line(match guild.nsfw_level {
				NsfwLevel::Default => "default".to_owned(),
				NsfwLevel::Explicit => "explicit".to_owned(),
				NsfwLevel::Safe => "safe".to_owned(),
				NsfwLevel::AgeRestricted => "age-restricted".to_owned(),
				_ => guild.nsfw_level.num().to_string(),
			});
	}
	
	if let Some(banner_url) = guild.banner_url() {
		builder.push_bold("Banner:")
			.push(' ')
			.push_named_link_safe("Click", banner_url)
			.push('\n');
	}

	if let Some(ref vanity_code) = guild.vanity_url_code {
		builder.push_bold("Vanity URL Code:")
			.push(' ')
			.push_mono_line_safe(vanity_code);
	}

	if let Some(splash_url) = guild.splash_url() {
		builder.push_bold("Splash:")
			.push(' ')
			.push_named_link_safe("Click", splash_url)
			.push('\n');
	}

	if let Some(ref description) = guild.description {
		builder.push_bold("Description:")
			.push(' ');

		if description.len() <= 128 {
			builder.push_line_safe(description);
		} else {
			builder.push_safe(&description[..128]).push_line("...");
		}
	}

	embed.field(&guild.name, builder.build(), false);
}

fn set_channel_info(channel: &Channel, embed: &mut CreateEmbed) {
	let mut builder = MessageBuilder::new();

	builder.push_bold("Snowflake:")
		.push(' ')
		.push_mono_line(channel.id().as_u64());
		
	builder.push_bold("Created At:")
		.push(' ')
		.push_line(channel.id().created_at().mention(SHORT_DATE_TIME));

	if channel.is_nsfw() {
		builder.push_bold_line("Is NSFW");
	}

	let name = match channel {
		Channel::Guild(guild_channel) => {
			builder.push_bold("Guild Channel Kind:")
				.push(' ')
				.push_line(guild_channel.kind.name());
			"#".to_owned() + guild_channel.name()
		}
		Channel::Private(private_channel) => {
			builder.push_bold("Recipient:")
				.push(' ')
				.user(&private_channel.recipient)
				.push('\n')
				.push_bold_line("Private Channel");
			"DM".to_owned()
		}
		Channel::Category(category_channel) => {
			builder.push_bold_line("Guild Category");
			category_channel.name().to_owned()
		}
    	_ => {
			"unknown channel type".to_owned()
		},
	};

	embed.field(name, builder.build(), false);
}

fn other_info_embed(id: ChannelId, embed: &mut CreateEmbed) -> &mut CreateEmbed {
	embed
		.description(MessageBuilder::new()
			.push_bold("Snowflake:")
			.push(' ')
			.push_mono_line(id.as_u64())
			
			.push_bold("Created At:")
			.push(' ')
			.push_line(id.created_at().mention(SHORT_DATE_TIME)))
		.color(ERROR_EMBED_COLOR)
}
