use serenity::{
    async_trait,
    model::{
        channel::{Message, Reaction},
        gateway::Ready,
    },
    client::{Context, EventHandler},
};

pub struct RoleHandler;

#[async_trait]
impl EventHandler for RoleHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, r: Reaction) {
    }

    async fn reaction_remove(&self, ctx: Context, r: Reaction) {
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready to manage roles!", ready.user.name);
    }
}
