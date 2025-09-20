
//! # ARITHMETIC LOGIC UNIT
//! 
//! ## There are three inputs for the ALU: A, B and an OPCODE.
//! A -> the first operand
//! B -> the second operand
//! OPCODE -> which operation we want to perform
//!
//! ## There are two outputs: RESULT and some FLAGS
//! RESULT -> the operation result
//! FLAGS -> some additonal information about the operation
//!
//!
//! # INSTRUCTION SET (OPCODE) consisting of 4 bits
//! 0000 : ADD -> a + b
//! 0001 : ADC -> a + b + 1 if last op. gen. a carry
//! 0010 : SUB -> a - b
//! 0011 : SBB -> a - b - 1 if last op. did not gen. a carry
//! 0100 : ONC -> inverts bits of b
//! 0101 : TWC -> inverts bits of b and adds 1
//! 0110 : AND -> bitwise and on bits of a and b
//! 0111 : OR  -> bitwise or on bits of a and b
//! 1000 : XOR -> bitwise xor on bits of a and b
//! 1001 : LSL -> logical left shift on b
//! 1010 : LSR -> logical right shift on b
//! 1011 : ASR -> arithmetic right shift on b
//! 1100 : ROL -> rotate left on b
//! 1101 : ROR -> rotate right on b
//! 1110 : RCL -> rotate left on b through carry
//! 1111 : RCR -> rotate right on b through carry


use std::rc::Rc;

use crate::types::alu::AluGateInputs;
use crate::types::lookahead::GPSingleOutputs;
use crate::logic::gates::{empty_and, empty_or, nand, not, xor};
use crate::logic::gates::implement::Gate;

// Builder function for a general adder gate for the ALU
// state1 -> A
// state2 -> B
// en_s1  -> enable A
// in_s2  -> invert B
// x1     -> OP code 1
// x2     -> OP code 2
// c_in   -> carry in

// (en) -> nand
// (en, x1) -> xor

pub fn general_gate_alu(inps: AluGateInputs) -> GPSingleOutputs {

    let AluGateInputs {
        state1,
        state2,
        enable1,
        invert2,
        opcode1,
        opcode2,
        c_in
    } = inps;

    let out_nand = nand(enable1, state1);
    let out_xnor = not(xor(invert2, state2));
    
    let mut and_gate = empty_and();
    and_gate.apply(out_nand.clone());
    and_gate.apply(opcode1);
    and_gate.apply(out_xnor.clone());

    let out_and = Rc::new(and_gate);

    
    let mut or_gate = empty_or();
    or_gate.apply(out_nand);
    or_gate.apply(opcode2);
    or_gate.apply(out_xnor);

    let out_nor1 = not(
        Rc::new(or_gate)
    );

    
    let mut or_gate = empty_or();
    or_gate.apply(out_and);
    or_gate.apply(out_nor1.clone());

    let out_nor2 = not(
        Rc::new(or_gate)
    );


    let out_xor = xor(c_in, out_nor2.clone());


    GPSingleOutputs {
        out: out_xor,
        propagate: out_nor2,
        generate: out_nor1
    }

}
