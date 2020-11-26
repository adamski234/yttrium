use std::collections::HashMap;
use serenity::model::id::{ChannelId, MessageId, UserId, RoleId};
use serenity::model::channel::ReactionType;

/// Enum describing the event type that should be used for interpretation
#[derive(Debug)]
pub enum EventType {
	Default,
	MemberJoin(MemberJoinEventInfo),
	MemberLeave(MemberLeaveEventInfo),
	Message(MessageEventInfo),
	MemberUpdate(MemberUpdateEventInfo),
	RoleCreate(RoleCreateEventInfo),
	RoleDelete(RoleDeleteEventInfo),
	RoleUpdate(RoleUpdateEventInfo),
	ChannelCreate(ChannelCreateEventInfo),
	ChannelDelete(ChannelDeleteEventInfo),
	ChannelUpdate(ChannelUpdateEventInfo),
	GuildUpdate(GuildUpdateEventInfo),
	VoiceUpdate(VoiceUpdateEventInfo),
	ReactionAdd(ReactionAddEventInfo),
	ReactionRemove(ReactionRemoveEventInfo),
}

#[derive(Debug)]
pub struct MessageEventInfo {
	pub channel_id: ChannelId,
	pub message_id: MessageId,
	pub user_id: UserId,
	pub trigger: String,
	pub parameter: String,
	pub split_parameters: HashMap<String, Vec<String>>,
}

impl MessageEventInfo {
	pub fn new(channel_id: ChannelId, message_id: MessageId, user_id: UserId, parameter: String, trigger: String) -> Self {
		return Self {
			channel_id: channel_id,
			message_id: message_id,
			user_id: user_id,
			trigger: trigger,
			parameter: parameter,
			split_parameters: HashMap::new(),
		};
	}
}

#[derive(Debug)]
pub struct MemberJoinEventInfo {
	pub user_id: UserId,
}

impl MemberJoinEventInfo {
	pub fn new(user_id: UserId) -> Self { 
		return Self { user_id };
	}
}

#[derive(Debug)]
pub struct MemberLeaveEventInfo {
	pub user_id: UserId,
}

impl MemberLeaveEventInfo {
	pub fn new(user_id: UserId) -> Self { 
		return Self { user_id };
	}
}

#[derive(Debug)]
pub struct MemberUpdateEventInfo {
	pub user_id: UserId,
}

impl MemberUpdateEventInfo {
	pub fn new(user_id: UserId) -> Self { 
		return Self { user_id };
	}
}

#[derive(Debug)]
pub struct RoleCreateEventInfo {
	pub role_id: RoleId,
}

impl RoleCreateEventInfo {
	pub fn new(role_id: RoleId) -> Self { 
		return Self { role_id };
	}
}

#[derive(Debug)]
pub struct RoleDeleteEventInfo {
	pub role_id: RoleId,
}

impl RoleDeleteEventInfo {
	pub fn new(role_id: RoleId) -> Self { 
		return Self { role_id };
	}
}

#[derive(Debug)]
pub struct RoleUpdateEventInfo {
	pub role_id: RoleId,
}

impl RoleUpdateEventInfo {
	pub fn new(role_id: RoleId) -> Self { 
		return Self { role_id };
	}
}

#[derive(Debug)]
pub struct ChannelCreateEventInfo {
	pub channel_id: ChannelId,
}

impl ChannelCreateEventInfo {
	pub fn new(channel_id: ChannelId) -> Self { 
		return Self { channel_id };
	}
}

#[derive(Debug)]
pub struct ChannelDeleteEventInfo {
	pub channel_id: ChannelId,
}

impl ChannelDeleteEventInfo {
	pub fn new(channel_id: ChannelId) -> Self { 
		return Self { channel_id };
	}
}

#[derive(Debug)]
pub struct ChannelUpdateEventInfo {
	pub channel_id: ChannelId,
}

impl ChannelUpdateEventInfo {
	pub fn new(channel_id: ChannelId) -> Self { 
		return Self { channel_id };
	}
}

#[derive(Debug)]
pub struct VoiceUpdateEventInfo {
	pub channel_id: ChannelId,
	pub user_id: UserId,
}

impl VoiceUpdateEventInfo {
	pub fn new(channel_id: ChannelId, user_id: UserId) -> Self { 
		return Self { channel_id, user_id };
	}
}

#[derive(Debug)]
pub struct GuildUpdateEventInfo {}

impl GuildUpdateEventInfo {
	pub fn new() -> Self { 
		return Self {};
	}
}

impl Default for GuildUpdateEventInfo {
	fn default() -> Self {
		return Self::new();
	}
}

#[derive(Debug)]
pub struct ReactionAddEventInfo {
	pub channel_id: ChannelId,
	pub message_id: MessageId,
	pub user_id: UserId,
	pub reaction_id: ReactionType,
}

impl ReactionAddEventInfo {
	pub fn new(channel_id: ChannelId, message_id: MessageId, user_id: UserId, reaction_id: ReactionType) -> Self { 
		return Self { channel_id, message_id, user_id, reaction_id };
	}
}

#[derive(Debug)]
pub struct ReactionRemoveEventInfo {
	pub channel_id: ChannelId,
	pub message_id: MessageId,
	pub user_id: UserId,
	pub reaction_id: ReactionType,
}

impl ReactionRemoveEventInfo {
	pub fn new(channel_id: ChannelId, message_id: MessageId, user_id: UserId, reaction_id: ReactionType) -> Self { 
		return Self { channel_id, message_id, user_id, reaction_id };
	}
}