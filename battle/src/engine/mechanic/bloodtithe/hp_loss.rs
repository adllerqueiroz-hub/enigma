const BASE_MAX: i32 = 24;
const DAMAGE_PER_POINT: i32 = 3000;

pub fn threshold(max: i32) -> i32 {
    (DAMAGE_PER_POINT - (max - BASE_MAX).max(0)).max(1)
}
