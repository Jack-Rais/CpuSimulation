
use std::rc::Rc;
use std::cell::Cell;

#[derive(Debug)]
pub enum Gate {
    Input(Rc<Cell<bool>>),
    Not(Rc<Gate>),
    And(Vec<Rc<Gate>>),
    Or(Vec<Rc<Gate>>),
}

impl Gate {

    pub fn constant(state: bool) -> Gate {
        Gate::Input(Rc::new(Cell::new(state)))
    }
    
    pub fn input(state: Rc<Cell<bool>>) -> Gate {
        Gate::Input(state)
    }

    pub fn input_empty(state: bool) -> Gate {
        Gate::Input(Rc::new(Cell::new(state)))
    }
    
    pub fn not(state: Rc<Gate>) -> Gate {
        Gate::Not(state)
    }

    pub fn and(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        
        let mut cont = Vec::new();
        cont.push(state1);
        cont.push(state2);
        
        Gate::And(cont)
    }

    pub fn empty_and() -> Gate {
        Gate::And(Vec::new())
    }

    pub fn or(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        
        let mut cont = Vec::new();
        cont.push(state1);
        cont.push(state2);
        
        Gate::Or(cont)
    }

    pub fn empty_or() -> Gate {
        Gate::Or(Vec::new())
    }

    pub fn nand(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        Gate::Not(
            Rc::new(Gate::and(
                state1, state2
            ))
        )
    }

    pub fn xor(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        Gate::or(
            Rc::new(Gate::and(
                Rc::new(Gate::not(state1.clone())),
                state2.clone()
            )),
            Rc::new(Gate::and(
                state1.clone(),
                Rc::new(Gate::not(state2.clone()))
            ))
        )
    }


    pub fn apply(self, state: Rc<Gate>) -> Self {

        match self {
            Gate::Input(_) => panic!("Value not changeable directly for an Input gate"),
            Gate::Not(_) => Gate::Not(state),
            Gate::And(mut a) => { a.push(state); Gate::And(a) },
            Gate::Or(mut a) => { a.push(state); Gate::Or(a) },
        }

    }

    pub fn eval(&self) -> bool {

        match self {
            Gate::Input(a) => a.get(),
            Gate::Not(a) => !a.eval(),
            Gate::And(a) => { for x in a.iter() { if !x.eval() { return false } } return true },
            Gate::Or(a) => { for x in a.iter() { if x.eval() { return true } } return false },
        }

    }

}

