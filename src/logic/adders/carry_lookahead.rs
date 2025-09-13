
use std::rc::Rc;

use crate::logic::gates::{and, empty_and, empty_or, xor, Gate, GateRef};
use crate::logic::adders::types::{
    CarryLookaheadUnitInputs,
    CarryLookaheadUnitGPOutputs,
    Adder4BitInputs,
    Adder4BitGPOutputs,
};


/// Builder function for a full adder with Carry Lookahead
///
/// # Outputs: (out_sum, propagate, generate)
pub fn full_adder_cl(state1: GateRef, state2: GateRef, carry: GateRef) -> (GateRef, GateRef, GateRef) {
    
    let prop = Rc::new(xor(state1.clone(), state2.clone()));
    let gener = Rc::new(and(state1, state2));

    let out = Rc::new(xor(prop.clone(), carry));

    (out, prop, gener)

}


/// Builder function for a 4 bit Carry Lookahead unit
///
/// # Inputs: (generates (with carry_in), propagates (with carry_in))
/// # Outputs: (carry_outs, propagate, generate)
pub fn carry_lookahead_unit(inps: CarryLookaheadUnitInputs) -> CarryLookaheadUnitGPOutputs {

    let CarryLookaheadUnitInputs { props, gens, carry_in } = inps;
    
    let mut prop_out: Option<GateRef> = None;
    let mut gener_out: Option<GateRef> = None;

    let mut result: Vec<GateRef> = Vec::with_capacity(3);

    for idx_carry in 0..4 {

        // The current carry turns on if one of the generate is on and
        // every propagate from that generate and the carry in on
        // C0 = G0 + (P0 ⋅ C0)
        // C1 = G1 + (P1 ⋅ G0) + (P1 ⋅ P0 ⋅ C0)
        // ...
        // C3 = G3 + (P3 ⋅ G2) + (P3 ⋅ P2 ⋅ G1) + (P3 ⋅ P2 ⋅ P1 ⋅ G0) + (P3 ⋅ P2 ⋅ P1 ⋅ P0 ⋅ C0)
        
        let mut curr_or = empty_or();

        for (look_forward, gener) in gens[0..idx_carry + 1].iter().rev().chain(std::iter::once(&carry_in)).enumerate() {
            
            // We do not create the and gate
            if look_forward == 0 { curr_or.apply(gener.clone()); }

            let mut curr_and = empty_and();
            curr_and.apply(gener.clone());

            for prop in props[idx_carry + 1 - look_forward..idx_carry + 1].iter() {
                curr_and.apply(prop.clone());
            }

            let final_and = Rc::new(curr_and);
            
            if look_forward == 3 {
                prop_out = Some(final_and);
            }
            else {
                curr_or.apply(final_and);
            }

        }

        let final_gen = Rc::new(curr_or);
        
        if idx_carry == 3 {
            gener_out = Some(final_gen);
        }
        else {
            result.push(final_gen);
        }

    }


    CarryLookaheadUnitGPOutputs {
        carry_outs: match result.try_into() { Ok(x) => x, _ => panic!("3 bit vector did not return 3 bits") },
        propagate: prop_out.expect("prop_out was never set"),
        generate: gener_out.expect("gen_out was never set"),
    }


}


/// Builder function for a 4 bit adder with Carry Lookahead
///
/// # Outputs: (4bit_sum, propagate, generate)
/// To calculate directly the carry_out you can do ((carry_in AND propagate) OR generate)
pub fn adder_4bit_cl(inps: Adder4BitInputs) -> Adder4BitGPOutputs {

    let Adder4BitInputs { num1, num2, carry_in } = inps;


    let gens: [GateRef; 4] = std::array::from_fn(|idx| {
        let x: GateRef = Rc::new(and(num1[3 - idx].clone(), num2[3 - idx].clone())); x
    });

    let props: [GateRef; 4] = std::array::from_fn(|idx| {
        let x: GateRef = Rc::new(xor(num1[3 - idx].clone(), num2[3 - idx].clone())); x
    });

    let CarryLookaheadUnitGPOutputs {carry_outs, propagate, generate} = carry_lookahead_unit(CarryLookaheadUnitInputs {
        props: &props,
        gens: &gens,
        carry_in: carry_in.clone()
    });

    let mut result: [GateRef; 4] = std::array::from_fn(|idx| {
        if idx == 0 { let x: GateRef = Rc::new(xor(props[idx].clone(), carry_in.clone())); return x; }
        let x: GateRef = Rc::new(xor(props[idx].clone(), carry_outs[idx - 1].clone())); x
    });
    result.reverse();
    
    Adder4BitGPOutputs {
        sum: result,
        propagate,
        generate
    }
}

