/// Appends a 1, then a copy of the first element.
/// CLUE: The borrow checker rejected `let first = &v[0]; v.push(1);`. What did `unsafe` change: the rule, or only the check?
/// CLUE: Where does `first` point after `push` grows the Vec?
/// CLUE: Run `cargo +nightly miri test`. Which line does Miri blame?
pub fn push_one_then_first(v: &mut Vec<i32>) {
    let first: &i32 = unsafe { &*v.as_ptr() };
    v.push(1);
    v.push(*first);
}
