#[derive(Debug, PartialEq)]
pub struct InvalidTransition;

/// An order moves Created -> Paid -> Shipped, and can be cancelled before it ships.
/// CLUE: Three bools give eight combinations. How many of them are real states?
/// CLUE: What does `paid: false, shipped: true, cancelled: true` mean?
#[derive(Debug, Default)]
pub struct Order {
    pub paid: bool,
    pub shipped: bool,
    pub cancelled: bool,
}

impl Order {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pay(&mut self) -> Result<(), InvalidTransition> {
        self.paid = true;
        Ok(())
    }

    pub fn ship(&mut self) -> Result<(), InvalidTransition> {
        if !self.paid {
            return Err(InvalidTransition);
        }
        self.shipped = true;
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), InvalidTransition> {
        self.cancelled = true;
        Ok(())
    }
}
