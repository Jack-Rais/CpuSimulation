
mod parts {
    pub mod gates;
    pub mod objects;
    pub mod operations;
}

use std::rc::Rc;

use crate::parts::objects::UnsignedInteger8;
use crate::parts::operations::Ops;

fn main() {
    
    let x = Rc::new(UnsignedInteger8::from_bits([
        false,
        false,
        true,
        false,
        false,
        true,
        true,
        false,
    ]));

    let y = Rc::new(UnsignedInteger8::from_bits([
        false,
        true,
        false,
        true,
        false,
        true,
        true,
        true,
    ]));

    
    println!("{} + {}", x.to_dec(), y.to_dec());
    
    let u = Ops::adder_8bit(
        &x,
        &y
    );

    if let Ops::Adder8Bit(sum, over) = &u {
        println!("{}, {}", sum.to_dec(), over.eval());
    }

}
