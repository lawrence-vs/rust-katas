/// Writes `new` into the place `slot` points at and returns what was there.
pub fn replace(slot: &i32, new: i32) -> i32 {
    let old = *slot;
    *slot = new;
    old
}
