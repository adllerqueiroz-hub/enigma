use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PulsingField {
    transferred: HashMap<i32, i32>,
}

impl PulsingField {
    pub fn transfer(&mut self, team: i32, amount: i32) -> i32 {
        if team == 0 || amount <= 0 {
            return self.transferred(team);
        }
        let value = self.transferred.entry(team).or_default();
        *value += amount;
        *value
    }

    pub fn transferred(&self, team: i32) -> i32 {
        self.transferred.get(&team).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_is_persistent_per_team() {
        let mut field = PulsingField::default();

        assert_eq!(field.transfer(1, 4), 4);
        assert_eq!(field.transfer(1, 6), 10);
        assert_eq!(field.transferred(2), 0);
    }
}
