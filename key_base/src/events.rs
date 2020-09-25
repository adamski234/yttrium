//This sadly ties the parser too strongly to the platform it's supposed to work on
#[derive(Debug)]
pub enum EventType {
	Default,
	MemberJoin(MemberJoinEventInfo),
	MemberLeave(MemberLeaveEventInfo),
	Message(MessageEventInfo),
	MemberUpdate,
	RoleCreate,
	RoleDelete,
	RoleUpdate,
	ChannelCreate,
	ChannelDelete,
	ChannelUpdate,
	ServerUpdate,
	VoiceUpdate,
	ReactionAdd,
	ReactionRemove,
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
		return Self { user_id }
	}
}

#[derive(Debug)]
pub struct MemberLeaveEventInfo {
	pub user_id: String,
}

impl MemberLeaveEventInfo {
	pub fn new(user_id: String) -> Self { 
		return Self { user_id }
	}
}
