use crate::{
    GameDB, character_level::CharacterLevel, character_rank::CharacterRank,
    character_talent::CharacterTalent, character_voice::CharacterVoice, skin::Skin,
    talent_scheme::TalentScheme, talent_style_cost::TalentStyleCost,
};

impl GameDB {
    pub fn max_faith(&self) -> i32 {
        self.friendless.iter().map(|row| row.friendliness).sum()
    }

    pub fn faith_percent(&self, faith: i32) -> i32 {
        let mut accumulated = 0;
        let mut percent = 0;

        for level in self.friendless.iter() {
            accumulated += level.friendliness;
            if faith < accumulated {
                return percent;
            }
            percent = level.percentage;
            if faith == accumulated {
                return percent;
            }
        }

        100
    }

    pub fn talent_scheme(&self, talent_id: i32, talent_mould: i32) -> Option<&TalentScheme> {
        self.talent_scheme
            .iter()
            .find(|row| row.talent_id == talent_id && row.talent_mould == talent_mould)
    }

    pub fn starting_character_level(&self, hero_id: i32) -> Option<&CharacterLevel> {
        self.character_level
            .iter()
            .filter(|row| row.hero_id == hero_id)
            .min_by_key(|row| row.level)
    }

    pub fn max_character_level(&self) -> i32 {
        self.character_level
            .iter()
            .map(|row| row.level)
            .max()
            .unwrap_or_default()
    }

    pub fn starting_character_rank(&self, hero_id: i32) -> Option<&CharacterRank> {
        self.character_rank
            .iter()
            .filter(|row| row.hero_id == hero_id)
            .min_by_key(|row| row.rank)
    }

    pub fn character_talent(&self, hero_id: i32, talent_id: i32) -> Option<&CharacterTalent> {
        self.character_talent
            .iter()
            .find(|row| row.hero_id == hero_id && row.talent_id == talent_id)
    }

    pub fn character_voices(&self, hero_id: i32) -> impl Iterator<Item = &CharacterVoice> {
        self.character_voice
            .iter()
            .filter(move |row| row.hero_id == hero_id)
    }

    pub fn default_character_skin(&self, hero_id: i32) -> Option<&Skin> {
        self.skin
            .iter()
            .filter(|row| row.character_id == hero_id)
            .min_by_key(|row| row.id)
    }

    pub fn talent_style_cost(&self, hero_id: i32, style_id: i32) -> Option<&TalentStyleCost> {
        self.talent_style_cost
            .iter()
            .find(|row| row.hero_id == hero_id && row.style_id == style_id)
    }
}
