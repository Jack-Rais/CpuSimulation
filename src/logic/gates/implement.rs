
use std::rc::Rc;
use std::cell::Cell;

pub trait Gate {
    fn eval(&self) -> bool;
    fn apply(&mut self, state: Rc<dyn Gate>);
}


pub struct Input {
    inp: Rc<Cell<bool>>
}
impl Input {

    pub fn new(state: Rc<Cell<bool>>) -> Self {
        Self { inp: state }
    }
}
impl Gate for Input {

    fn eval(&self) -> bool {
        self.inp.get()
    }

    fn apply(&mut self, _state: Rc<dyn Gate>) {
        panic!("Value not changeable for an Input type");
    }

}


pub struct Not {
    inp: Rc<dyn Gate>
}
impl Not {
    pub fn new(state: Rc<dyn Gate>) -> Self {
        Not { inp: state }
    }
}
impl Gate for Not {

    fn eval(&self) -> bool {
        !self.inp.eval()
    }

    fn apply(&mut self, state: Rc<dyn Gate>) {
        self.inp = state;
    }

}


pub struct And {
    inps: Vec<Rc<dyn Gate>>
}
impl And {
    pub fn new(state: Vec<Rc<dyn Gate>>) -> Self {
        Self { inps: state }
    }
}
impl Gate for And {

    fn eval(&self) -> bool {
        for inp in self.inps.iter() {
            if !inp.eval() { return false }
        }

        true
    }

    fn apply(&mut self, state: Rc<dyn Gate>) {
        self.inps.push(state);
    }

}


pub struct Or {
    inps: Vec<Rc<dyn Gate>>
}
impl Or {
    pub fn new(state: Vec<Rc<dyn Gate>>) -> Self {
        Self { inps: state }
    }
}
impl Gate for Or {

    fn eval(&self) -> bool {
        for inp in self.inps.iter() {
            if inp.eval() { return true }
        }

        false
    }

    fn apply(&mut self, state: Rc<dyn Gate>) {
        self.inps.push(state);
    }

}
