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
	pub channel_id: String,
	pub message_id: String,
	pub user_id: String,
	pub trigger: String,
}

impl MessageEventInfo {
	pub fn new(channel_id: String, message_id: String, user_id: String, trigger: String) -> Self {
		return Self { channel_id, message_id, user_id, trigger };
	}
}

#[derive(Debug)]
pub struct MemberJoinEventInfo {
	pub user_id: String,
}

impl MemberJoinEventInfo {
	pub fn new(user_id: String) -> Self { 
		return Self { user_id };
	}
}

#[derive(Debug)]
pub struct MemberLeaveEventInfo {
	pub user_id: String,
}

impl MemberLeaveEventInfo {
	pub fn new(user_id: String) -> Self { 
		return Self { user_id };
	}
}

#[derive(Debug)]
pub struct MemberUpdateEventInfo {
	pub user_id: String,
}

impl MemberUpdateEventInfo {
	pub fn new(user_id: String) -> Self { 
		return Self { user_id };
	}
}

#[derive(Debug)]
pub struct RoleCreateEventInfo {
	pub role_id: String,
}

impl RoleCreateEventInfo {
	pub fn new(role_id: String) -> Self { 
		return Self { role_id };
	}
}

#[derive(Debug)]
pub struct RoleDeleteEventInfo {
	pub role_id: String,
}

impl RoleDeleteEventInfo {
	pub fn new(role_id: String) -> Self { 
		return Self { role_id };
	}
}

#[derive(Debug)]
pub struct RoleUpdateEventInfo {
	pub role_id: String,
}

impl RoleUpdateEventInfo {
	pub fn new(role_id: String) -> Self { 
		return Self { role_id };
	}
}

#[derive(Debug)]
pub struct ChannelCreateEventInfo {
	pub channel_id: String,
}

impl ChannelCreateEventInfo {
	pub fn new(channel_id: String) -> Self { 
		return Self { channel_id };
	}
}

#[derive(Debug)]
pub struct ChannelDeleteEventInfo {
	pub channel_id: String,
}

impl ChannelDeleteEventInfo {
	pub fn new(channel_id: String) -> Self { 
		return Self { channel_id };
	}
}

#[derive(Debug)]
pub struct ChannelUpdateEventInfo {
	pub channel_id: String,
}

impl ChannelUpdateEventInfo {
	pub fn new(channel_id: String) -> Self { 
		return Self { channel_id };
	}
}

#[derive(Debug)]
pub struct VoiceUpdateEventInfo {
	pub channel_id: String,
	pub user_id: String,
}

impl VoiceUpdateEventInfo {
	pub fn new(channel_id: String, user_id: String) -> Self { 
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
	pub channel_id: String,
	pub message_id: String,
	pub user_id: String,
	pub reaction_id: String,
}

impl ReactionAddEventInfo {
	pub fn new(channel_id: String, message_id: String, user_id: String, reaction_id: String) -> Self { 
		return Self { channel_id, message_id, user_id, reaction_id };
	}
}

#[derive(Debug)]
pub struct ReactionRemoveEventInfo {
	pub channel_id: String,
	pub message_id: String,
	pub user_id: String,
	pub reaction_id: String,
}

impl ReactionRemoveEventInfo {
	pub fn new(channel_id: String, message_id: String, user_id: String, reaction_id: String) -> Self { 
		return Self { channel_id, message_id, user_id, reaction_id };
	}
}