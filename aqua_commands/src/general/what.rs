use crate::internal::prelude::*;

#[command]
#[num_args(0)]
pub async fn what(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
	msg.channel_id.say(&ctx, "TODO: Channel & Guild Info").await?;
	Ok(())
}
