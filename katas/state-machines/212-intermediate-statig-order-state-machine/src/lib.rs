use statig::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Pay,
    Ship,
    Cancel,
}

#[derive(Default)]
pub struct Order;

#[state_machine(initial = "State::created()")]
impl Order {
    #[state]
    fn created(event: &Event) -> Outcome<State> {
        match event {
            Event::Pay => Transition(State::paid()),
            Event::Cancel => Transition(State::cancelled(*event)),
            Event::Ship => Handled,
        }
    }

    #[state]
    fn paid(event: &Event) -> Outcome<State> {
        match event {
            Event::Ship => Transition(State::shipped()),
            Event::Cancel => Transition(State::cancelled(*event)),
            Event::Pay => Handled,
        }
    }

    #[state]
    fn shipped(event: &Event) -> Outcome<State> {
        match event {
            Event::Cancel => Transition(State::cancelled(*event)),
            Event::Pay | Event::Ship => Handled,
        }
    }

    #[state]
    fn cancelled(_event: &Event) -> Outcome<State> {
        Handled
    }
}
