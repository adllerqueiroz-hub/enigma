use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserStats {
    pub user_id: i64,
    pub first_charge: bool,
    pub total_charge_amount: i64,
    pub is_first_login: bool,
    pub user_tag: String,
}

impl From<UserStats> for sonettobuf::StatInfoPush {
    fn from(stats: UserStats) -> Self {
        sonettobuf::StatInfoPush {
            frist_charge: stats.first_charge.then_some(true),
            total_charge_amount: (stats.total_charge_amount != 0)
                .then_some(stats.total_charge_amount),
            is_first_login: stats.is_first_login.then_some(true),
            player_info: None,
            user_tag: (!stats.user_tag.is_empty()).then_some(stats.user_tag),
        }
    }
}
