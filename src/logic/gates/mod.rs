
pub mod implement;

use implement::{Gate, Input, Not, And, Or};

use std::{cell::Cell, rc::Rc};



/// Builder function for a static Input
pub fn constant(state: bool) -> Input {
    Input::new(Rc::new(Cell::new(state)))
}


/// Builder function for a dynamic Input
pub fn input(state: Rc<Cell<bool>>) -> Input {
    Input::new(state)
}


/// Builder function for a Not gate
pub fn not(state: Rc<dyn Gate>) -> Not {
    Not::new(state)
}


/// Builder function for an And gate
pub fn and(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>) -> And {
    
    let mut inps = Vec::with_capacity(2);

    inps.push(state1);
    inps.push(state2);

    And::new(inps)
}


/// Builder function for an empty And gate
pub fn empty_and() -> And {
    And::new(Vec::new())
}


/// Builder function for an Or gate
pub fn or(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>) -> Or {

    let mut inps = Vec::with_capacity(2);

    inps.push(state1);
    inps.push(state2);

    Or::new(inps)
}


/// Builder function for an empty Or gate
pub fn empty_or() -> Or {
    Or::new(Vec::new())
}


/// Builder function for a Nand gate
pub fn nand(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>) -> Not {
    Not::new(
        Rc::new(and(state1, state2))
    )
}


/// Builder function for a Xor gate
pub fn xor(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>) -> Or {
    
    or(
        Rc::new(and(state1.clone(), state2.clone())),
        Rc::new(and(
            Rc::new(not(state1.clone())),
            Rc::new(not(state2.clone()))
        ))
    )

}


