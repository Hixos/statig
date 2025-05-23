#![allow(unused)]

use statig::blocking::{self, *};

#[derive(Default)]
pub struct Blinky {
    led: bool,
}

#[derive(Debug)]
pub enum State {
    On,
    Off,
}

pub struct Event;

/// The `StateMachine` trait needs to be implemented on the type that will be
/// the shared storage for the state machine.
impl IntoStateMachine for Blinky {
    /// The enum that represents the state.
    type State = State;

    /// We are not using any superstates for this state machine, so we set it to `()`.
    type Superstate<'sub> = ();

    /// The event type that will be submitted to the state machine.
    type Event<'evt> = Event;

    type Context<'ctx> = i64;

    /// The initial state of the state machine.
    fn initial() -> Self::State {
        State::Off
    }

    /// This method is called on every transition of the state machine.
    fn after_transition(&mut self, source: &Self::State, target: &Self::State, context: &mut i64) {
        println!("transitioned from {source:?} to {target:?} with context {context:#?}");
    }
}

impl blocking::State<Blinky> for State {
    fn call_handler(&mut self, blinky: &mut Blinky, event: &Event, _: &mut i64) -> Response<Self> {
        match self {
            State::On => blinky.on(event),
            State::Off => blinky.off(event),
        }
    }

    fn call_entry_action(&mut self, shared_storage: &mut Blinky, context: &mut <Blinky as IntoStateMachine>::Context<'_>) {
        match self {
            State::On => shared_storage.entry_on(context),
            State::Off => {}
        }
    }
}

impl Blinky {
    fn on(&mut self, event: &Event) -> Response<State> {
        self.led = false;
        // Transition to the `off` state.
        Transition(State::Off)
    }

    fn off(&mut self, event: &Event) -> Response<State> {
        self.led = true;
        // Transition to the `on` state.
        Transition(State::On)
    }

    fn entry_on(&mut self, context: &mut <Blinky as IntoStateMachine>::Context<'_>) {
        // This method is called when entering the state.
        println!("Entering state {context}");
    }
}

fn main() {
    let mut ctx= 3i64;

    let mut state_machine = Blinky::default().uninitialized_state_machine().init_with_context(&mut ctx);
    state_machine.handle_with_context(&Event, &mut ctx);
}
