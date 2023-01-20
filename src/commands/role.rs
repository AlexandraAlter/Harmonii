use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{Reaction, ReactionType},
        event::ResumedEvent,
        gateway::Ready,
    },
};
use tracing::{error, info};

use crate::conf::*;

pub struct RoleHandler;

impl RoleHandler {}

#[async_trait]
impl EventHandler for RoleHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is ready to manage roles!", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resuming role management");
    }

    async fn reaction_add(&self, ctx: Context, r: Reaction) {
        let data = ctx.data.read().await;

        let config = some_or_return!(data.get::<ConfigContainer>(), || error!("No config found"));

        let guild_id = some_or_return!(r.guild_id, || error!("No guild found"));
        let guild_opt = config.guilds.iter().find(|g| g.id == guild_id);
        let guild_conf = some_or_return!(guild_opt, || error!("No guild config found"));
        let role_man = some_or_return!(&guild_conf.roles);

        if !role_man.watched.contains(&r.channel_id) {
            return;
        }

        let (e_id, e) = match r.emoji {
            ReactionType::Custom { id, .. } => (Some(id), None),
            ReactionType::Unicode(u) => (None, Some(u)),
            _ => return info!("Unknown emoji"),
        };

        let role_opt = role_man
            .available
            .iter()
            .find(|r| r.emoji_id == e_id && r.emoji == e);
        let role = match role_opt {
            Some(r) => r,
            None => {
                let e_either = e_id.map(|i| i.to_string()).or(e);
                let e_name = e_either.as_ref().map_or("?", |x| &**x);
                error!("No matching role found for emoji: {}", e_name);
                return;
            }
        };

        let user_id = some_or_return!(r.user_id, || error!("No user found"));
        let member_res = guild_id.member(&ctx, user_id).await;
        let mut member = result_or_return!(member_res, |_e| error!("No member found"));

        let res = member.add_role(&ctx, role.id).await;
        result_or_return!(res, |_e| error!("Unable to add role"));

        let default_name = "?".to_string();
        let name = role.name.as_ref().unwrap_or(&default_name);
        info!("Added role {} to {}", name, member.display_name());
    }

    async fn reaction_remove(&self, ctx: Context, r: Reaction) {
        let data = ctx.data.read().await;

        let config = some_or_return!(data.get::<ConfigContainer>(), || error!("No config found"));

        let guild_id = some_or_return!(r.guild_id, || error!("No guild found"));
        let guild_opt = config.guilds.iter().find(|g| g.id == guild_id);
        let guild_conf = some_or_return!(guild_opt, || error!("No guild config found"));
        let role_man = some_or_return!(&guild_conf.roles);

        if !role_man.watched.contains(&r.channel_id) {
            return;
        }

        let (e_id, e) = match r.emoji {
            ReactionType::Custom { id, .. } => (Some(id), None),
            ReactionType::Unicode(u) => (None, Some(u)),
            _ => return info!("Unknown emoji"),
        };

        let role_opt = role_man
            .available
            .iter()
            .find(|r| r.emoji_id == e_id && r.emoji == e);
        let role = some_or_return!(role_opt, || error!("No matching role found for emoji"));

        let user_id = some_or_return!(r.user_id, || error!("No user found"));
        let member_res = guild_id.member(&ctx, user_id).await;
        let mut member = result_or_return!(member_res, |_e| error!("No member found"));

        let res = member.remove_role(&ctx, role.id).await;
        result_or_return!(res, |_e| error!("Unable to add role"));

        let default_name = "?".to_string();
        let name = role.name.as_ref().unwrap_or(&default_name);
        info!("Removed role {} from {}", name, member.display_name());
    }
}
