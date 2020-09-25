#[derive(Debug)]
pub enum EventInfo {
	Default,
	MemberJoin,
	MemberLeave,
	Message,
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