
use std::cell::Cell;
use std::rc::Rc;

use super::objects::UnsignedInteger8;
use super::gates::Gate;

// Creates an half adder and returns (sum, carry)
pub fn half_adder(state1: &Rc<Gate>, state2: &Rc<Gate>) -> (Rc<Gate>, Rc<Gate>) {
    
    let sum = Gate::nand(state1.clone(), state2.clone());
    let carry = Gate::and(state1.clone(), state2.clone());

    (Rc::new(sum), Rc::new(carry))

}

// Ripple-carry full_adder
// Creates a full adder and returns (sum, carry)
pub fn full_adder_rc(state1: &Rc<Gate>, state2: &Rc<Gate>, carry: Rc<Gate>) -> (Rc<Gate>, Rc<Gate>) {
    
    let out_xor = Rc::new(Gate::xor(state1.clone(), state2.clone()));
    let out_and = Rc::new(Gate::and(state1.clone(), state2.clone()));

    let out_xor2 = Rc::new(Gate::xor(out_xor.clone(), carry.clone()));
    let out_and2 = Rc::new(Gate::and(out_xor.clone(), carry.clone()));

    let out_or = Rc::new(Gate::or(out_and.clone(), out_and2.clone()));

    (out_xor2, out_or)

}



// Carry-lookahead full adder
// Creates a full adder with carry-lookahead unit and returns (sum, propagate, generate)
pub fn full_adder_cl(state1: &Rc<Gate>, state2: &Rc<Gate>, carry: Rc<Gate>) -> (Rc<Gate>, Rc<Gate>, Rc<Gate>) {
    
    let prop = Rc::new(Gate::xor(state1.clone(), state2.clone()));
    let gener = Rc::new(Gate::and(state1.clone(), state2.clone()));

    let out = Rc::new(Gate::xor(prop.clone(), carry.clone()));

    (out, prop, gener)

}

// 4 Bit version of a carry-lookahead adder
pub fn adder_4bit_cl(num1: [Rc<Gate>; 4], num2: [Rc<Gate>; 4], carry: Rc<Gate>) -> ([Rc<Gate>; 4], Rc<Gate>) {

    let mut result = Vec::with_capacity(4);
    
    let mut geners = Vec::with_capacity(5);
    let mut props = Vec::with_capacity(5);

    geners.push(carry.clone());
    props.push(carry.clone());

    let mut past_carry = carry.clone();

    for num_cl in 0..4 {
       
        // Create the first part of the adder
        let (curr_out, prop, gener) = full_adder_cl(
            &num1[3 - num_cl],
            &num2[3 - num_cl],
            past_carry.clone()
        );
        result.push(curr_out);

        // We need to keep track of the propagate and generate signals
        props.push(prop.clone());
        geners.push(gener.clone());
        
        let mut curr_carry = Gate::empty_or(); // The Or gate that feeds into the next carry

        // Create the row of ANDs
        for num_and in 0..num_cl+1 {
            
            let mut curr_and = Gate::empty_and();
            println!("{num_and}");
            
            // The current carry turns on if one of the generate is on and
            // every propagate from that generate and the carry in on
            // C4 = G3 + (P3 ⋅ G2) + (P3 ⋅ P2 ⋅ G1) + (P3 ⋅ P2 ⋅ P1 ⋅ G0) + (P3 ⋅ P2 ⋅ P1 ⋅ P0 ⋅ C0)

            curr_and = curr_and.apply(
                geners[geners.len() - num_and - 1].clone()
            );

            let props_len = props.len();
            for num_past_propagate in 0..num_and {
                curr_and = curr_and.apply(props[props_len - num_past_propagate - 1].clone());
            }

            curr_carry = curr_carry.apply(Rc::new(curr_and));

        }
        
        past_carry = Rc::new(curr_carry);

    }
    
    // Reverse the result to take the MSB to the left
    result.reverse();

    (
        result.try_into().expect("A 4 bit vector was not of 4 bits"),
        past_carry
    )

}

/// A 4bit version of a Ripple-carry adder that returns an (out, generate, propagate)
pub fn adder_4bit_clgp(num1: [Rc<Gate>; 4], num2: [Rc<Gate>; 4], carry_in: Rc<Gate>) -> ([Rc<Gate>; 4], Rc<Gate>, Rc<Gate>) {
    
    let mut result = Vec::with_capacity(4);
    let mut out_prop = Rc::new(Gate::Input(Rc::new(Cell::new(false))));

    let mut geners = Vec::with_capacity(5);
    let mut props = Vec::with_capacity(5);

    geners.push(carry_in.clone());
    props.push(carry_in.clone());

    let mut past_carry = carry_in.clone();

    for num_cl in 0..4 {
       
        // Create the first part of the adder
        let (curr_out, prop, gener) = full_adder_cl(
            &num1[3 - num_cl],
            &num2[3 - num_cl],
            past_carry.clone()
        );
        result.push(curr_out);

        // We need to keep track of the propagate and generate signals
        props.push(prop.clone());
        geners.push(gener.clone());
        
        let mut curr_carry = Gate::empty_or(); // The Or gate that feeds into the next carry

        // Create the row of ANDs 
        for num_and in 0..num_cl+1 {
            
            let mut curr_and = Gate::empty_and();
            println!("{num_and}");
            
            // The current carry turns on if one of the generate is on and
            // every propagate from that generate and the carry in on
            // C4 = G3 + (P3 ⋅ G2) + (P3 ⋅ P2 ⋅ G1) + (P3 ⋅ P2 ⋅ P1 ⋅ G0) + (P3 ⋅ P2 ⋅ P1 ⋅ P0 ⋅ C0)

            curr_and = curr_and.apply(
                geners[geners.len() - num_and - 1].clone()
            );

            let props_len = props.len();
            for num_past_propagate in 0..num_and {
               curr_and = curr_and.apply(props[props_len - num_past_propagate - 1].clone());
            }

            if num_and == num_cl && num_cl == 3 {
                out_prop = Rc::new(curr_and);
            }
            else {
                curr_carry = curr_carry.apply(Rc::new(curr_and));
            }

        }
        
        past_carry = Rc::new(curr_carry);

    }

    // Reverse the result to take the MSB to the left
    result.reverse();

    (
        result.try_into().expect("A 4 bit vector was not of 4 bits"),
        out_prop,
        past_carry
    )

}

// Ripple-carry adder
// Creates an 8 bit adder and returns (sum: UnsignedInteger8, carry: Gate)
pub fn adder_8bit_rc(num1: &Rc<UnsignedInteger8>, num2: &Rc<UnsignedInteger8>) -> (Rc<UnsignedInteger8>, Rc<Gate>) {
    
    let mut result = Vec::with_capacity(8);
    let mut carry = Rc::new(Gate::constant(false));

    for (bit1, bit2) in num1.value.iter().rev().zip(num2.value.iter().rev()) {

        let (sum, carry_in) = full_adder_rc(&bit1.clone(), &bit2.clone(), carry.clone());

        result.push(sum.clone());
        carry = carry_in.clone();

    }

    result.reverse();
    let array: [Rc<Gate>; 8] = result
        .try_into()
        .expect("UnsignedInteger8 did not return 8 Gates arrays");
    
    (Rc::new(UnsignedInteger8::new(array)), carry)

}

// Carry-lookahead adder
// Crates an 8 bit adder with carry-lookahead and returns (sum: UnsignedInteger8, carry: Gate)
// pub fn adder_8bit_cl(num1: &rc<unsignedinteger8>, num2: &rc<unsignedinteger8>) -> (rc<unsignedinteger8>, rc<gate>) {
//     
//         
//
// }
