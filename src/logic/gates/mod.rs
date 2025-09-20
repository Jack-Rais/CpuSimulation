
pub mod implement;

pub use implement::Gate;
use implement::{Input, Not, And, Or};

use std::{cell::Cell, rc::Rc};


pub type GateRef = Rc<dyn Gate>;


pub fn constant_naked(state: bool) -> Input {
    Input::new(Rc::new(Cell::new(state)))
}

/// Builder function for a static Input
pub fn constant(state: bool) -> GateRef {
    Rc::new(constant_naked(state))
}


/// Builder function for a dynamic Input
pub fn input(state: Rc<Cell<bool>>) -> impl Gate {
    Input::new(state)
}

/// Builder function for a Not gate
pub fn not(state: GateRef) -> GateRef {
    Rc::new(Not::new(state))
}


/// Builder function for an And gate
pub fn and(state1: GateRef, state2: GateRef) -> GateRef {
    
    let inps = vec![
        state1,
        state2
    ];

    Rc::new(And::new(inps))
}


/// Builder function for an empty And gate
pub fn empty_and() -> impl Gate {
    And::new(Vec::new())
}


/// Builder function for an Or gate
pub fn or(state1: GateRef, state2: GateRef) -> GateRef {

    let inps = vec![
        state1,
        state2
    ];

    Rc::new(Or::new(inps))
}


/// Builder function for an empty Or gate
pub fn empty_or() -> impl Gate {
    Or::new(Vec::new())
}


/// Builder function for a Nand gate
pub fn nand(state1: GateRef, state2: GateRef) -> GateRef {
    Rc::new(Not::new(
        and(state1, state2)
    ))
}


/// Builder function for a Xor gate
pub fn xor(state1: GateRef, state2: GateRef) -> GateRef {
    
    or(
        and(
            not(state1.clone()),
            state2.clone()
        ),
        and(
            state1,
            not(state2)
        )
    )

}


