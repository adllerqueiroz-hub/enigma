use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct DiceHeroChapter {
    pub user_id: i64,
    pub chapter: i32,
    pub current_hero_id: i32,
    pub relic_ids: String,
    pub skill_card_ids: String,
    pub pass_level_ids: String,
    pub reward_items_json: String,
    pub updated_at: i64,
}
