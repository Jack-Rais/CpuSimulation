
mod parts {
    pub mod gates;
    pub mod objects;
    pub mod operations;
}

use std::rc::Rc;
use std::cell::Cell;

use parts::gates::{Gate};


fn main() {
    
    let poss = [
        ( true, true ),
        ( true, false ),
        ( false, true ),
        ( false, false )
    ];

    let input1 = Rc::new(Cell::new(true));
    let input2 = Rc::new(Cell::new(true));

    let gate = Gate::and(
        Rc::new(Gate::input(input1.clone())),
        Rc::new(Gate::input(input2.clone()))
    );


    for &(x, y) in poss.iter() {
        input1.set(x);
        input2.set(y);

        println!("x: {}, y: {} -> {}", x, y, gate.eval());
    }

    // println!("{}", gate);
    // println!("{}", gate.eval());

}
