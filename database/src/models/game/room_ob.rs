use sonettobuf::{FormulaInfo, ProductionLineInfo, RoomHeroData, RoomSkinInfo};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RoomFormula {
    pub formula_id: i32,
    pub count: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct RoomProductionLine {
    pub line_id: i32,
    pub formula_id: i32,
    pub finish_count: i32,
    pub next_finish_time: i32,
    pub pause_time: i32,
    pub level: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct RoomSkin {
    pub part_id: i32,
    pub skin_id: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct RoomHero {
    pub hero_id: i32,
    pub current_faith: i32,
    pub next_refresh_time: i32,
    pub skin: i32,
    pub current_minute: i32,
}

impl RoomFormula {
    pub fn into_proto(self) -> impl Iterator<Item = FormulaInfo> {
        let count = self.count.max(0);
        (0..count).map(move |_| FormulaInfo {
            id: Some(self.formula_id),
        })
    }
}

impl From<RoomProductionLine> for ProductionLineInfo {
    fn from(line: RoomProductionLine) -> Self {
        ProductionLineInfo {
            id: Some(line.line_id),
            formula_id: Some(line.formula_id),
            finish_count: Some(line.finish_count),
            next_finish_time: Some(line.next_finish_time),
            pause_time: Some(line.pause_time),
            level: Some(line.level),
        }
    }
}

impl From<RoomSkin> for RoomSkinInfo {
    fn from(skin: RoomSkin) -> Self {
        RoomSkinInfo {
            id: Some(skin.part_id),
            skin_id: Some(skin.skin_id),
        }
    }
}

impl From<RoomHero> for RoomHeroData {
    fn from(hero: RoomHero) -> Self {
        RoomHeroData {
            hero_id: Some(hero.hero_id),
            current_faith: Some(hero.current_faith),
            next_refresh_time: Some(hero.next_refresh_time),
            skin: Some(hero.skin),
            current_minute: Some(hero.current_minute),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_formula_expands_to_repeated_formula_infos() {
        let infos = RoomFormula {
            formula_id: 2002001,
            count: 3,
        }
        .into_proto()
        .collect::<Vec<_>>();

        assert_eq!(infos.len(), 3);
        assert!(infos.iter().all(|info| info.id == Some(2002001)));
    }

    #[test]
    fn room_formula_negative_count_generates_no_infos() {
        let infos = RoomFormula {
            formula_id: 2002001,
            count: -1,
        }
        .into_proto()
        .collect::<Vec<_>>();

        assert!(infos.is_empty());
    }
}
