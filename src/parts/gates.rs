
use std::rc::Rc;
use std::cell::Cell;

pub enum Gate {
    Input(Rc<Cell<bool>>),
    Not(Rc<Gate>),
    And(Rc<Gate>, Rc<Gate>),
    Or(Rc<Gate>, Rc<Gate>),
}

impl Gate {
    
    pub fn input(state: Rc<Cell<bool>>) -> Gate {
        Gate::Input(state)
    }
    
    pub fn not(state: Rc<Gate>) -> Gate {
        Gate::Not(state)
    }

    pub fn and(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        Gate::And(state1, state2)
    }

    pub fn or(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        Gate::Or(state1, state2)
    }

    pub fn nand(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {
        Gate::Not(
            Rc::new(Gate::And(
                state1, state2
            ))
        )
    }

    pub fn xor(state1: Rc<Gate>, state2: Rc<Gate>) -> Gate {

    }



    pub fn eval(&self) -> bool {

        match self {
            Gate::Input(a) => a.get(),
            Gate::Not(a) => !a.eval(),
            Gate::And(a, b) => a.eval() && b.eval(),
            Gate::Or(a, b) => a.eval() || b.eval(),
        }

    }

}

// impl std::fmt::Display for Gate {
//     
//     fn fmt<T>(&self, f: &mut T) -> std::fmt::Result {
//         match self {
//             Gate::Input(a) => write!(f, "{}", if a.get() {"1"} else {"0"}),
//             Gate::Not(a) => write!(f, "¬({})", a),
//             Gate::And(a, b) => write!(f, "({} ∧ {})", a, b),
//             Gate::Or(a, b) => write!(f, "({} ∨ {})", a, b),
//         }
//     }
//
// }




