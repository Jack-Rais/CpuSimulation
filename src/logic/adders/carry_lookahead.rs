
use std::rc::Rc;

use crate::logic::gates::Gate;


/// Builder function for a full adder with Carry Lookahead
///
/// # Outputs: (out_sum, propagate, generate)
pub fn full_adder_cl(state1: &Rc<Gate>, state2: &Rc<Gate>, carry: Rc<Gate>) -> (Rc<Gate>, Rc<Gate>, Rc<Gate>) {
    
    let prop = Rc::new(Gate::xor(state1.clone(), state2.clone()));
    let gener = Rc::new(Gate::and(state1.clone(), state2.clone()));

    let out = Rc::new(Gate::xor(prop.clone(), carry.clone()));

    (out, prop, gener)

}


/// Builder function for a 4 bit adder with Carry Lookahead
///
/// # Outputs: (4bit_sum, propagate, generate)
/// To calculate directly the carry_out you can do ((carry_in AND propagate) OR generate)
pub fn adder_4bit_cl(num1: [Rc<Gate>; 4], num2: [Rc<Gate>; 4], carry: Rc<Gate>) -> ([Rc<Gate>; 4], Rc<Gate>, Rc<Gate>) {

    let mut result = Vec::with_capacity(4);
    let mut prop_out = Rc::new(Gate::input_empty(false));
    
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
            
            // The last or gate that feeds into the last and gate is the propagate signal
            if num_and == 3 {
                prop_out = Rc::new(curr_and);
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
        prop_out,
        past_carry
    )

}



