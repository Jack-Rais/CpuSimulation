
use std::rc::Rc;

use crate::logic::gates::{constant, and, or, nand, xor};
use crate::logic::gates::GateRef;


/// Builder function for a half adder
///
/// # Outputs: (out_sum, carry_out)
pub fn half_adder(state1: GateRef, state2: GateRef) -> (GateRef, GateRef) {
    
    let sum = nand(state1.clone(), state2.clone());
    let carry = and(state1, state2);

    (Rc::new(sum), Rc::new(carry))

}


/// Builder function for a full adder with Ripple Carry
///
/// # Outputs (out_sum, carry_out)
pub fn full_adder_rc(state1: GateRef, state2: GateRef, carry: GateRef) -> (GateRef, GateRef) {
    
    let out_xor = Rc::new(xor(state1.clone(), state2.clone()));
    let out_and = Rc::new(and(state1, state2));

    let out_xor2 = Rc::new(xor(out_xor.clone(), carry.clone()));
    let out_and2 = Rc::new(and(out_xor, carry));

    let out_or = Rc::new(or(out_and, out_and2));

    (out_xor2, out_or)

}


/// Builder function for an 8 bit adder with Ripple Carry
///
/// # Outputs (8bit_integer_sum, carry)
pub fn adder_8bit_rc(num1: [GateRef; 8], num2: [GateRef; 8]) -> ([GateRef; 8], GateRef) {
    
    let mut result = Vec::with_capacity(8);
    let mut carry: GateRef = Rc::new(constant(false));

    for (bit1, bit2) in num1.iter().rev().zip(num2.iter().rev()) {

        let (sum, carry_in) = full_adder_rc(bit1.clone(), bit2.clone(), carry);

        result.push(sum);
        carry = carry_in;

    }

    result.reverse();
    let array: [GateRef; 8] = match result.try_into() {
        Err(_) => panic!("An 8 gates array did not return 8 values"),
        Ok(x) => x
    };
    
    (array, carry)

}

