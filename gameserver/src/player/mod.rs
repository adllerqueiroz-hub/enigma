pub mod battle;
pub mod state;

pub use battle::BattleManager;
use logic::{
    activity::ActivityManager, collection::CollectionManager, inventory::InventoryManager,
    red_dot::RedDotManager, task::TaskManager,
};
pub use state::PlayerState;

#[derive(Debug, Clone)]
pub struct Player {
    pub id: i64,
    pub state: PlayerState,
    pub activity: ActivityManager,
    pub battle: BattleManager,
    pub collection: CollectionManager,
    pub inventory: InventoryManager,
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
            inventory: InventoryManager::new(id),
            red_dot: RedDotManager::new(id),
            tasks: TaskManager::new(id),
        }
    }
}
