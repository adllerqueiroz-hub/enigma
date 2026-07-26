use sonettobuf;
use sqlx::FromRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TowerType {
    Normal = 1,
    Boss = 2,
    Limited = 3,
}

impl TowerType {
    pub const fn id(self) -> i32 {
        self as i32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TowerConstId {
    MaxMopUpTimes = 101,
    BalanceBossLevel = 118,
    CustomTalentPlanCount = 119,
    TeachBossLevel = 121,
}

impl TowerConstId {
    pub const fn id(self) -> i32 {
        self as i32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TowerStatus {
    Ready = 1,
    Open = 2,
}

impl TowerStatus {
    pub const fn id(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserTowerInfo {
    pub user_id: i64,
    pub mop_up_times: i32,
    pub trial_hero_season: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct TowerOpen {
    pub tower_type: i32,
    pub tower_id: i32,
    pub status: i32,
    pub round: i32,
    pub next_time: i64,
    pub tower_start_time: i64,
    pub task_end_time: i64,
}

impl From<TowerOpen> for sonettobuf::TowerOpenNo {
    fn from(t: TowerOpen) -> Self {
        sonettobuf::TowerOpenNo {
            r#type: Some(t.tower_type),
            tower_id: Some(t.tower_id),
            status: Some(t.status),
            round: Some(t.round),
            next_time: Some(t.next_time),
            tower_start_time: Some(t.tower_start_time),
            task_end_time: Some(t.task_end_time),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeroInfo {
    pub hero_id: i32,
    pub equip_uids: Vec<i64>,
    pub trial_id: i32,
}

impl From<HeroInfo> for sonettobuf::HeroNo {
    fn from(h: HeroInfo) -> Self {
        sonettobuf::HeroNo {
            hero_id: Some(h.hero_id),
            equip_uid: h.equip_uids,
            trial_id: Some(h.trial_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TalentPlanInfo {
    pub plan_id: i32,
    pub talent_point: i32,
    pub talent_ids: Vec<i32>,
    pub plan_name: String,
}

impl From<TalentPlanInfo> for sonettobuf::TalentPlanNo {
    fn from(t: TalentPlanInfo) -> Self {
        sonettobuf::TalentPlanNo {
            plan_id: Some(t.plan_id),
            talent_point: Some(t.talent_point),
            talent_ids: t.talent_ids,
            plan_name: Some(t.plan_name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssistBossInfo {
    pub boss_id: i32,
    pub level: i32,
    pub talent_plans: Vec<TalentPlanInfo>,
    pub use_talent_plan: i32,
}

#[derive(Debug, Clone)]
pub struct ActiveTalentPlan {
    pub boss_level: i32,
    pub plan_id: i32,
    pub talent_point: i32,
    pub talent_ids: Vec<i32>,
}

impl From<AssistBossInfo> for sonettobuf::AssistBossNo {
    fn from(b: AssistBossInfo) -> Self {
        sonettobuf::AssistBossNo {
            id: Some(b.boss_id),
            level: Some(b.level),
            talent_plans: b.talent_plans.into_iter().map(Into::into).collect(),
            use_talent_plan: Some(b.use_talent_plan),
        }
    }
}
