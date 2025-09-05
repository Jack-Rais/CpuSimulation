
mod logic {
    pub mod objects;
    pub mod operations;
    pub mod gates;
}

mod cpu {
    pub mod alu;
}

use std::{cell::Cell, rc::Rc};

use crate::logic::gates::Gate;
use crate::logic::operations::adder_4bit_cl;



fn to_arr(state: u8) -> [bool; 4] {
    
    if state > 15 { panic!("Non è possibile convertire un numero maggiore di 16") }

    let mut bits = [false; 4];
    for i in 0..4 {
        bits[3 - i] = (state & (1 << i)) != 0;
    }

    bits
}


fn to_rc(state: [bool; 4]) -> [Rc<Gate>; 4] {
    std::array::from_fn(|i| Rc::new(Gate::Input(Rc::new(Cell::new(state[i])))))
}


fn bit4_intou8(inp: [Rc<Gate>; 4]) -> u8 {
    let mut out = 0;
    for (n, num) in inp.iter().rev().enumerate() {
        out += 2u8.pow(n as u32) * num.eval() as u8;
    }
    out
}


fn main() {
    
    let num1 = 9;
    let num2 = 7;

    let n1 = to_rc(to_arr(num1));
    let n2 = to_rc(to_arr(num2));

    println!("{:?}", n1.clone().map(|x| x.eval()));
    println!("{:?}", n2.clone().map(|x| x.eval()));

    let res = adder_4bit_cl(n1, n2, Rc::new(Gate::Input(Rc::new(Cell::new(false)))));
    
    println!("{:?} - {}", bit4_intou8(res.0), res.1.eval());

}
