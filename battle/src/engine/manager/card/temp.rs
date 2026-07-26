use sonettobuf::CardInfo;

pub fn temp_card(skill_id: i32) -> CardInfo {
    CardInfo {
        uid: Some(0),
        skill_id: Some(skill_id),
        card_effect: Some(0),
        temp_card: Some(true),
        enchants: Vec::new(),
        card_type: Some(0),
        hero_id: Some(0),
        status: Some(0),
        target_uid: Some(0),
        extra_info: None,
        energy: Some(0),
        extra_infos: Vec::new(),
        area_red_or_blue: Some(0),
        heat_id: Some(0),
        music_note: None,
        card_dataes: Vec::new(),
    }
}

pub fn precast_card(owner_uid: i64, skill_id: i32) -> CardInfo {
    CardInfo {
        uid: Some(owner_uid),
        hero_id: config::try_get()
            .and_then(|db| db.skill.get(skill_id))
            .map(|skill| skill.hero_id),
        card_type: Some(sonettobuf::card_info::CardType::Skill3 as i32),
        ..temp_card(skill_id)
    }
}

pub fn selected_precast_card(owner_uid: i64, hero_id: i32, skill_id: i32) -> CardInfo {
    CardInfo {
        uid: Some(owner_uid),
        hero_id: Some(hero_id),
        ..temp_card(skill_id)
    }
}
