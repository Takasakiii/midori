use std::ops::BitAnd;

use poise::{
    async_trait,
    serenity_prelude::{GuildId, Member, Permissions, User},
};

use crate::commands::{CommandResult, Context};

pub trait UserBotExtension {
    fn get_bot_user(&self) -> User;
    fn get_bot_member(&self, guild_id: GuildId) -> Option<Member>;
    fn get_bot_member_for_current_guild(&self) -> Option<Member>;
    fn check_bot_permissions(&self, permissions: Permissions) -> bool;
}

impl<'a> UserBotExtension for Context<'a> {
    fn get_bot_user(&self) -> User {
        let discord = self.discord();
        let current_user = discord.cache.current_user();
        discord
            .cache
            .user(current_user.id)
            .expect("Current user not found")
    }

    fn get_bot_member(&self, guild_id: GuildId) -> Option<Member> {
        let discord = self.discord();
        let current_user = discord.cache.current_user();
        discord.cache.member(guild_id, current_user.id)
    }

    fn get_bot_member_for_current_guild(&self) -> Option<Member> {
        let guild = self.guild_id()?;

        self.get_bot_member(guild)
    }

    fn check_bot_permissions(&self, permissions_check: Permissions) -> bool {
        let guild_member = self.get_bot_member_for_current_guild();

        if let Some(member) = guild_member {
            if let Ok(permissions) = member.permissions(self.discord()) {
                permissions.bitand(permissions_check) > Permissions::empty()
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub trait UserExtension {
    fn get_guild_member(&self, guild_id: GuildId) -> Option<Member>;
    fn check_user_permissions(&self, permissions_check: Permissions) -> bool;
    fn author_member(&self) -> Option<Member>;
}

impl<'a> UserExtension for Context<'a> {
    fn check_user_permissions(&self, permissions_check: Permissions) -> bool {
        let member = self.author_member();

        if let Some(member) = member {
            if let Ok(permissions) = member.permissions(self.discord()) {
                permissions.bitand(permissions_check) > Permissions::empty()
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_guild_member(&self, guild_id: GuildId) -> Option<Member> {
        self.discord().cache.member(guild_id, self.author().id)
    }

    fn author_member(&self) -> Option<Member> {
        self.get_guild_member(self.guild_id()?)
    }
}

#[async_trait]
pub trait MessageExtension {
    async fn send_error(&self, message: &str) -> CommandResult;
}

#[async_trait]
impl<'a> MessageExtension for Context<'a> {
    async fn send_error(&self, message: &str) -> CommandResult {
        self.say(format!("❌ **|** {}", message)).await?;
        Ok(())
    }
}
