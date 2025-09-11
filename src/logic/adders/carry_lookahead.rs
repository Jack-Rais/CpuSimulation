
use std::rc::Rc;

use crate::logic::gates::implement::Gate;
use crate::logic::gates::{and, constant, empty_and, empty_or, xor};


/// Builder function for a full adder with Carry Lookahead
///
/// # Outputs: (out_sum, propagate, generate)
pub fn full_adder_cl(state1: Rc<dyn Gate>, state2: Rc<dyn Gate>, carry: Rc<dyn Gate>) -> (Rc<dyn Gate>, Rc<dyn Gate>, Rc<dyn Gate>) {
    
    let prop = Rc::new(xor(state1.clone(), state2.clone()));
    let gener = Rc::new(and(state1.clone(), state2.clone()));

    let out = Rc::new(xor(prop.clone(), carry.clone()));

    (out, prop, gener)

}


/// Builder function for a 4 bit Carry Lookahead unit
///
/// # Inputs: (generates (with carry_in), propagates (with carry_in))
/// # Outputs: (carry_outs, propagate, generate)
pub fn carry_lookahead_unit(props: &[Rc<dyn Gate>; 4], gens: &[Rc<dyn Gate>; 5]) -> ([Rc<dyn Gate>; 3], Rc<dyn Gate>, Rc<dyn Gate>) {
    
    let mut prop_out: Rc<dyn Gate> = Rc::new(constant(false));
    let mut gener_out: Rc<dyn Gate> = Rc::new(constant(false));

    let mut result: Vec<Rc<dyn Gate>> = Vec::with_capacity(4);

    for idx_carry in 0..4 {

        // The current carry turns on if one of the generate is on and
        // every propagate from that generate and the carry in on
        // C4 = G3 + (P3 ⋅ G2) + (P3 ⋅ P2 ⋅ G1) + (P3 ⋅ P2 ⋅ P1 ⋅ G0) + (P3 ⋅ P2 ⋅ P1 ⋅ P0 ⋅ C0)

        let mut curr_carry = empty_or();
        curr_carry.apply(gens[idx_carry + 1].clone());

        for idx_and in 0..idx_carry+1 {

            let mut curr_and = empty_and();
            curr_and.apply(gens[idx_carry - idx_and].clone());
            
            for propagate_sig in props[(idx_carry - idx_and)..(idx_carry)].iter() {
                curr_and.apply(propagate_sig.clone());
            }

            if idx_and == 3 {
                prop_out = Rc::new(curr_and);
            }
            else {
                curr_carry.apply(Rc::new(curr_and));
            }

        }

        let out_carry: Rc<dyn Gate> = Rc::new(curr_carry);

        if idx_carry == 3 {
            gener_out = out_carry.clone() 
        }

        result[3 - idx_carry] = out_carry;

    }

    (
        match result.try_into() { Ok(x) => x, _ => panic!("4 bit vector did not return 4 values")},
        prop_out,
        gener_out
    )

}



/// Builder function for a 4 bit adder with Carry Lookahead
///
/// # Outputs: (4bit_sum, propagate, generate)
/// To calculate directly the carry_out you can do ((carry_in AND propagate) OR generate)
pub fn adder_4bit_cl(num1: &[Rc<dyn Gate>; 4], num2: &[Rc<dyn Gate>; 4], carry_in: Rc<dyn Gate>) -> ([Rc<dyn Gate>; 4], Rc<dyn Gate>, Rc<dyn Gate>) {

    let gens: [Rc<dyn Gate>; 5] = std::array::from_fn(|idx| {
        if idx == 0 { return carry_in.clone() }
        Rc::new(and(num1[3 - idx].clone(), num2[3 - idx].clone()))
    });

    let props: [Rc<dyn Gate>; 4] = std::array::from_fn(|idx| {
        let x: Rc<dyn Gate> = Rc::new(xor(num1[3 - idx].clone(), num2[3 - idx].clone())); x
    });

    let (carries, prop_sig, gen_sig) = carry_lookahead_unit(&props, &gens);

    let mut result: [Rc<dyn Gate>; 4] = std::array::from_fn(|idx| {
        let x: Rc<dyn Gate> = Rc::new(xor(props[idx].clone(), carries[idx].clone())); x
    });
    result.reverse();

    (
        match result.try_into() { Ok(x) => x, _ => panic!("4 bit vector did not return 4 values")},
        prop_sig,
        gen_sig
    )
}

