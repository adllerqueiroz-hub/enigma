#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum CopostConstId {
    ActivityEndTime = 3,
    ActivityStartTime = 6,
}

impl CopostConstId {
    pub const fn id(self) -> i32 {
        self as i32
    }
}
