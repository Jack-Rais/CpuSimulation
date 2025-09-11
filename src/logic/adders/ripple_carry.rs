
use std::rc::Rc;

use crate::logic::gates::{constant, and, or, nand, xor};
use crate::logic::gates::implement::Gate;


/// Builder function for a half adder
///
/// # Outputs: (out_sum, carry_out)
pub fn half_adder(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>) -> (Rc<dyn Gate>, Rc<dyn Gate>) {
    
    let sum = nand(state1.clone(), state2.clone());
    let carry = and(state1.clone(), state2.clone());

    (Rc::new(sum), Rc::new(carry))

}


/// Builder function for a full adder with Ripple Carry
///
/// # Outputs (out_sum, carry_out)
pub fn full_adder_rc(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>, carry: Rc<dyn Gate>) -> (Rc<dyn Gate>, Rc<dyn Gate>) {
    
    let out_xor = Rc::new(xor(state1.clone(), state2.clone()));
    let out_and = Rc::new(and(state1.clone(), state2.clone()));

    let out_xor2 = Rc::new(xor(out_xor.clone(), carry.clone()));
    let out_and2 = Rc::new(and(out_xor.clone(), carry.clone()));

    let out_or = Rc::new(or(out_and.clone(), out_and2.clone()));

    (out_xor2, out_or)

}


/// Builder function for an 8 bit adder with Ripple Carry
///
/// # Outputs (8bit_integer_sum, carry)
pub fn adder_8bit_rc(num1: [Rc<dyn Gate>; 8], num2: [Rc<dyn Gate>; 8]) -> ([Rc<dyn Gate>; 8], Rc<dyn Gate>) {
    
    let mut result = Vec::with_capacity(8);
    let mut carry: Rc<dyn Gate> = Rc::new(constant(false));

    for (bit1, bit2) in num1.iter().rev().zip(num2.iter().rev()) {

        let (sum, carry_in) = full_adder_rc(bit1.clone(), bit2.clone(), carry.clone());

        result.push(sum.clone());
        carry = carry_in.clone();

    }

    result.reverse();
    let array: [Rc<dyn Gate>; 8] = match result.try_into() {
        Err(_) => panic!("An 8 gates array did not return 8 values"),
        Ok(x) => x
    };
    
    (array, carry)

}

