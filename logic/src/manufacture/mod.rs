mod building;
mod info;
mod production;

pub use building::*;
pub use info::*;
pub use production::*;

pub struct CostUpdate<T> {
    pub reply: T,
    pub item_ids: Vec<u32>,
    pub currency_ids: Vec<(i32, i32)>,
    pub material_changes: Vec<(u32, u32, i32)>,
}

#[cfg(test)]
mod test;
