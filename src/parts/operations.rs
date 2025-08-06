
use std::rc::Rc;
use super::gates::Gate;


pub enum Ops {
    HalfAdder(Rc<Gate>, Rc<Gate>),
}

impl Ops {

    pub fn half_adder(state1: &Rc<Gate>, state2: &Rc<Gate>) -> Ops {
        
        let sum = Gate::nand(state1.clone(), state2.clone());
        let carry = Gate::and(state1.clone(), state2.clone());

        Ops::HalfAdder(
            Rc::new(sum),
            Rc::new(carry)
        )
    }

}
