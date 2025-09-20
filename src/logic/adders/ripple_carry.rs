
use crate::logic::gates::{and, constant, nand, or, xor};
use crate::logic::gates::GateRef;
use crate::types::ripple::{
    GeneralSingleInputs,
    GeneralSingleCarryInputs,
    GeneralSingleCarryOutputs,
    General8BitInputs,
    General8BitCarryOutputs
};

pub trait Gate {
    fn eval(&mut self);
}


/// Builder function for a half adder
///
/// # Outputs: (out_sum, carry_out)
pub fn half_adder(inps: GeneralSingleInputs) -> GeneralSingleCarryOutputs {
    
    let GeneralSingleInputs { state1, state2 } = inps;

    GeneralSingleCarryOutputs {
        out_sum: nand(state1.clone(), state2.clone()),
        carry_out: and(state1, state2)
    }

}


/// Builder function for a full adder with Ripple Carry
///
/// # Outputs (out_sum, carry_out)
pub fn full_adder_rc(inps: GeneralSingleCarryInputs) -> GeneralSingleCarryOutputs {
    
    let GeneralSingleCarryInputs { state1, state2, carry_in } = inps;

    let out_xor = xor(state1.clone(), state2.clone());
    let out_and = and(state1, state2);

    let out_and2 = and(out_xor.clone(), carry_in.clone());

    GeneralSingleCarryOutputs {
        out_sum: xor(out_xor, carry_in),
        carry_out: or(out_and, out_and2),
    }

}


/// Builder function for an 8 bit adder with Ripple Carry
///
/// # Outputs (8bit_integer_sum, carry)
pub fn adder_8bit_rc(inps: General8BitInputs) -> General8BitCarryOutputs {
    
    let General8BitInputs { num1, num2 } = inps;

    let mut result = Vec::with_capacity(8);
    let mut carry: GateRef = constant(false);

    for (bit1, bit2) in num1.iter().rev().zip(num2.iter().rev()) {

        let GeneralSingleCarryOutputs { out_sum, carry_out } = full_adder_rc(GeneralSingleCarryInputs {
            state1: bit1.clone(), 
            state2: bit2.clone(), 
            carry_in: carry
        });

        result.push(out_sum);
        carry = carry_out;

    }

    result.reverse();
    let array: [GateRef; 8] = match result.try_into() {
        Err(_) => panic!("An 8 gates array did not return 8 values"),
        Ok(x) => x
    };
    
    General8BitCarryOutputs {
        out_sum: array,
        carry_out: carry,
    }

}

