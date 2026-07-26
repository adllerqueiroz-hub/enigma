pub mod activity;
pub mod battle;
pub mod collection;
pub mod red_dot;
pub mod state;
pub mod tasks;

pub use activity::ActivityManager;
pub use battle::BattleManager;
pub use collection::CollectionManager;
pub use red_dot::RedDotManager;
pub use state::PlayerState;
pub use tasks::TaskManager;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Player {
    pub id: i64,
    pub state: PlayerState,
    pub activity: ActivityManager,
    pub battle: BattleManager,
    pub collection: CollectionManager,
    pub red_dot: RedDotManager,
    pub tasks: TaskManager,
}

impl Player {
    pub fn new(id: i64, state: PlayerState) -> Self {
        Self {
            id,
            state,
            activity: ActivityManager::new(id),
            battle: BattleManager::default(),
            collection: CollectionManager::new(id),
            red_dot: RedDotManager::new(id),
            tasks: TaskManager::new(id),
        }
    }
}
