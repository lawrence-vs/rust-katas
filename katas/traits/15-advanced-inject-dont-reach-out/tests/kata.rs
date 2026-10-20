use kata_traits_15::{Clock, greeting};

struct Fixed(u32);

impl Clock for Fixed {
    fn hour(&self) -> u32 {
        self.0
    }
}

#[test]
fn morning() {
    assert_eq!(greeting(&Fixed(9)), "good morning");
}

#[test]
fn afternoon() {
    assert_eq!(greeting(&Fixed(15)), "good afternoon");
}
