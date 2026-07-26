mod chat;
mod friends;

use sonettobuf::GetAssistBonusReply;

#[derive(Clone, Copy, Debug)]
pub struct SocialManager {
    player_id: i64,
}

impl SocialManager {
    pub fn new(player_id: i64) -> Self {
        Self { player_id }
    }

    pub fn assist_bonus(&self) -> GetAssistBonusReply {
        GetAssistBonusReply {
            assist_bonus: Some(0),
            has_receive_assist_bonus: Some(0),
        }
    }
}
