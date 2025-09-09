
use std::rc::Rc;

use crate::logic::gates::Gate;


/// Builder function for a half adder
///
/// # Outputs: (out_sum, carry_out)
pub fn half_adder(state1: Rc<Gate>, state2: Rc<Gate>) -> (Rc<Gate>, Rc<Gate>) {
    
    let sum = Gate::nand(state1.clone(), state2.clone());
    let carry = Gate::and(state1.clone(), state2.clone());

    (Rc::new(sum), Rc::new(carry))

}


/// Builder function for a full adder with Ripple Carry
///
/// # Outputs (out_sum, carry_out)
pub fn full_adder_rc(state1: Rc<Gate>, state2: Rc<Gate>, carry: Rc<Gate>) -> (Rc<Gate>, Rc<Gate>) {
    
    let out_xor = Rc::new(Gate::xor(state1.clone(), state2.clone()));
    let out_and = Rc::new(Gate::and(state1.clone(), state2.clone()));

    let out_xor2 = Rc::new(Gate::xor(out_xor.clone(), carry.clone()));
    let out_and2 = Rc::new(Gate::and(out_xor.clone(), carry.clone()));

    let out_or = Rc::new(Gate::or(out_and.clone(), out_and2.clone()));

    (out_xor2, out_or)

}


/// Builder function for an 8 bit adder with Ripple Carry
///
/// # Outputs (8bit_integer_sum, carry)
pub fn adder_8bit_rc(num1: [Rc<Gate>; 8], num2: [Rc<Gate>; 8]) -> ([Rc<Gate>; 8], Rc<Gate>) {
    
    let mut result = Vec::with_capacity(8);
    let mut carry = Rc::new(Gate::constant(false));

    for (bit1, bit2) in num1.iter().rev().zip(num2.iter().rev()) {

        let (sum, carry_in) = full_adder_rc(bit1.clone(), bit2.clone(), carry.clone());

        result.push(sum.clone());
        carry = carry_in.clone();

    }

    result.reverse();
    let array: [Rc<Gate>; 8] = result
        .try_into()
        .expect("UnsignedInteger8 did not return 8 Gates arrays");
    
    (array, carry)

}

