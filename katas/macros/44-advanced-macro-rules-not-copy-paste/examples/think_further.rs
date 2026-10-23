//! 44 advanced: `macro_rules!`, not copy-paste
//!
//! Think further, from the kata README:
//! 1. The caller has its own `Option` or a `mod core` in scope where they invoke your macro. Which names in your expansion would resolve to theirs, and how do `::core::…` paths avoid that?
//! 2. A sixth type needs a different `checked_add`, for example a quantity capped at a maximum lot size. Would you add a macro arm, or hand-write that one impl? What does each do to "written once"?
//! 3. Your macro calls `Self(minor)`. What error does a caller get if they pass a struct with a named field, and where does rustc point: at their call or inside your macro?
//! 4. When does a declarative macro stop being enough, so that a `#[derive(Amount)]` proc macro is worth its extra crate?

// Scratch file for the Think further and In the wild items. Delete before the PR.
// Run with: cargo run --example think_further

// One macro arm writes checked arithmetic once for every newtype.
macro_rules! checked_amount {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(u64);

        impl $name {
            pub fn checked_add(self, other: $name) -> Option<$name> {
                self.0.checked_add(other.0).map($name)
            }
        }
    };
}

checked_amount!(Cents);
checked_amount!(Sats);

// Think further 2: a cap is a rule that differs per type, so it stays hand-written.
#[derive(Debug, PartialEq, Eq)]
struct Lots(u64);

impl Lots {
    const MAX: u64 = 1_000;

    fn checked_add(self, other: Lots) -> Option<Lots> {
        let sum = self.0.checked_add(other.0)?;
        (sum <= Self::MAX).then_some(Lots(sum))
    }
}

fn main() {
    println!("cents: {:?}", Cents(10).checked_add(Cents(5)));
    println!("sats overflow: {:?}", Sats(u64::MAX).checked_add(Sats(1)));
    println!("lots over cap: {:?}", Lots(900).checked_add(Lots(200)));

    // Think further 4: a derive macro would be worth it once many types need the same impl.
}
