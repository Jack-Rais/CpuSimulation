
use std::{cell::Cell, rc::Rc};
use super::gates::Gate;


pub struct UnsignedInteger8 {
    pub value: [Rc<Gate>; 8]
}

impl UnsignedInteger8 {
    
    pub fn new(value: [Rc<Gate>; 8]) -> Self {
        Self { value }
    }

    pub fn new_empty() -> Self {
        let value = std::array::from_fn(|_| Rc::new(Gate::Input(Rc::new(Cell::new(true)))));
        Self { value }
    }

    pub fn from_bits(value: [bool; 8]) -> Self {
        Self { value: value.map(|x| Rc::new(Gate::Input(Rc::new(Cell::new(x))))) }
    }

    pub fn set(&mut self, val: Rc<Gate>, pos: usize) {
        
        match self.value.get_mut(pos) {
            Some(v) => *v = val,
            _ => panic!("La posizione è fuori dal massimo consentito")
        }

    }

    pub fn to_dec(&self) -> u8 {
        self.value.iter().rev().enumerate().fold(0u8, |acc, (i, bit)| {
            let bit_value = if bit.eval() { 1 } else { 0 };
            acc.saturating_add(bit_value << i)
        })
    }

}
