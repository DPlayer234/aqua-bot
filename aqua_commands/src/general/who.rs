use crate::internal::prelude::*;
use aqua_util::time::{TimestampMention, SHORT_DATE_TIME};

#[command]
#[num_args(1)]
pub async fn who(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let user_id: UserId = args.single()?;

	if let Some(guild_id) = msg.guild_id {
		who_member_by_id(&ctx, &msg, guild_id, user_id).await
	} else {
		who_user_by_id(&ctx, &msg, user_id).await
	}
}

/* Grab the user/member by their ID, then delegate to further methods below */

async fn who_user_by_id(ctx: &Context, msg: &Message, user_id: UserId) -> CommandResult {
	let user = user_id.to_user(&ctx).await?;
	who_user(&ctx, &msg, user).await
}

async fn who_member_by_id(ctx: &Context, msg: &Message, guild_id: GuildId, user_id: UserId) -> CommandResult {
	match guild_id.member(&ctx, user_id).await {
		// If we found the member, no problem!
		Ok(member)
			=> who_member(&ctx, &msg, member).await,

		// Specifically retry if the error we got was a NOT_FOUND HTTP error
		Err(Error::Http(ref box_err))
		if box_err.status_code() == Some(StatusCode::NOT_FOUND)
			=> who_user_by_id(&ctx, &msg, user_id).await,

		Err(err) => Err(Box::new(err))
	}
}

/* Send an embed based on a grabbed user/member */

async fn who_user(ctx: &Context, msg: &Message, user: User) -> CommandResult {
	msg.channel_id.send_message(&ctx.http, |m| m
		.embed(|e| who_user_embed(&user, e))).await?;
	Ok(())
}

async fn who_member(ctx: &Context, msg: &Message, member: Member) -> CommandResult {
	msg.channel_id.send_message(&ctx.http, |m| m
		.embed(|e| who_member_embed(&ctx, &member, e))).await?;
	Ok(())
}

/* Format the embeds */

fn who_user_embed<'a>(user: &User, embed: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
	set_user_info(user, embed);

	embed.color(aqua_util::DEFAULT_EMBED_COLOR)
}

fn who_member_embed<'a>(ctx: &Context, member: &Member, embed: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
	set_user_info(&member.user, embed);
	set_member_info(&member, embed);

	embed.color(member.colour(&ctx).unwrap_or(aqua_util::DEFAULT_EMBED_COLOR))
}

fn set_user_info(user: &User, embed: &mut CreateEmbed) {
	let mut builder = MessageBuilder::new();

	builder.push_bold("Snowflake:")
		.push(' ')
		.push_mono_line(user.id.as_u64());

	builder.push_bold("Created At:")
		.push(' ')
		.push_line(user.created_at().mention(SHORT_DATE_TIME));

	if let Some(avatar_url) = user.avatar_url() {
		builder.push_bold("Avatar:")
			.push(' ')
			.push_named_link_safe("Click", avatar_url)
			.push('\n');
	} else {
		builder.push_bold_line("No Avatar");
	}

	builder.push_bold(if user.bot { "Bot Account:" } else { "User Account:" })
		.push(' ')
		.user(user);

	embed.author(|a| a.name(format!("{}#{:04}", user.name, user.discriminator)))
		.thumbnail(user.face())
		.description(builder.build());
}

fn set_member_info(member: &Member, embed: &mut CreateEmbed) {
	let mut builder = MessageBuilder::new();

	if let Some(ref nickname) = member.nick {
		builder.push_bold("Nickname:")
			.push(' ')
			.push_line_safe(nickname);
	} else {
		builder.push_bold_line("No Nickname");
	}

	let joined_at_text
		= if let Some(ref joined_at) = member.joined_at {
			joined_at.mention(SHORT_DATE_TIME)
		} else {
			"???".into()
		};

	builder.push_bold("Joined At:")
		.push(' ')
		.push_line(joined_at_text);

	if let Some(avatar_url) = member.avatar_url() {
		builder.push_bold("Avatar:")
			.push(' ')
			.push_named_link_safe("Click", avatar_url)
			.push('\n');
	}

	embed.field("Guild Member", builder.build(), false);
}
