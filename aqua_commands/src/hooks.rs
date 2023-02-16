use std::num::{ParseIntError, ParseFloatError};

use crate::imports::*;
use serenity::Error;
use serenity::framework::standard::{CommandResult, ArgError};
use serenity::framework::standard::DispatchError::{self, *};
use serenity::framework::standard::macros::hook;
use aqua_util::ERROR_EMBED_COLOR;

#[hook]
pub async fn after(ctx: &Context, msg: &Message, _: &str, result: CommandResult) {
	if let Err(err) = result {
		// We check the error type to give a better error message to the user
		// Check parse errors:
		if err.is::<ArgError<UserIdParseError>>() {
			send_error(&ctx, &msg, "Invalid user ID or mention.").await;
		} else if err.is::<ArgError<ChannelIdParseError>>() {
			send_error(&ctx, &msg, "Invalid channel ID or mention.").await;
		} else if err.is::<ArgError<RoleIdParseError>>() {
			send_error(&ctx, &msg, "Invalid role ID or mention.").await;
		} else if err.is::<ArgError<ParseIntError>>() {
			send_error(&ctx, &msg, "Invalid integer value.").await;
		} else if err.is::<ArgError<ParseFloatError>>() {
			send_error(&ctx, &msg, "Invalid numberic value.").await;
		} else if let Some(Error::Http(ref box_err)) = err.downcast_ref::<Error>() {
			// Specifically handle HTTP errors
			send_error(&ctx, &msg, MessageBuilder::new().push_safe(box_err)).await;
		} else {
			// If none of the above match, print a generic error message
			println!("Command Error: {}", err);
			send_error(&ctx, &msg, MessageBuilder::new().push_bold("Error: ").push_mono_safe(err)).await;
		}
	}

	async fn send_error(ctx: &Context, msg: &Message, content: impl ToString) {
		_ = msg.channel_id.send_message(&ctx, |m| m
			.add_embed(|e| e
				.description(content)
				.color(ERROR_EMBED_COLOR)
			)
		).await;
	}
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _command_name: &str) {
	match error {
		CheckFailed(_, reason) => {
			send_message(&ctx, &msg, reason).await;
		}
		OnlyForDM => {
			send_message(&ctx, &msg, "This command can only be used in DMs.").await;
		}
		OnlyForGuilds => {
			send_message(&ctx, &msg, "This command can only be used in guilds.").await;
		}
		OnlyForOwners => {
			send_message(&ctx, &msg, "This command can only be used by bot owners.").await;
		}
		LackingRole => {
			send_message(&ctx, &msg, "You don't have the required role to use this command.").await;
		}
		LackingPermissions(_) => {
			send_message(&ctx, &msg, "You don't have the required permissions to use this command.").await;
		}
		NotEnoughArguments { min, given } => {
			send_message(&ctx, &msg, format!("Got too few arguments. (Got {} but expected >= {}.)", given, min)).await;
		}
		TooManyArguments { max, given } => {
			send_message(&ctx, &msg, format!("Got too many arguments. (Got {} but expected <= {}.)", given, max)).await;
		}
		_ => {
			let text = format!("Command failed: ```{:?}```", error);
			send_message(&ctx, &msg, &text).await;
			println!("{}", text)
		}
	}

	async fn send_message(ctx: &Context, msg: &Message, text: impl ToString) {
		_ = msg.channel_id.send_message(&ctx, |m| m.content(text)).await;
	}
}
