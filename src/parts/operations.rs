
use std::rc::Rc;

use super::objects::UnsignedInteger8;
use super::gates::Gate;


pub enum Ops {
    HalfAdder(Rc<Gate>, Rc<Gate>),
    FullAdder(Rc<Gate>, Rc<Gate>),
    Adder8Bit(Rc<UnsignedInteger8>, Rc<Gate>),
}

impl Ops {

    pub fn half_adder(state1: &Rc<Gate>, state2: &Rc<Gate>) -> Ops {
        
        let sum = Gate::nand(state1.clone(), state2.clone());
        let carry = Gate::and(state1.clone(), state2.clone());

        Ops::HalfAdder(
            Rc::new(sum),
            Rc::new(carry)
        )
    }

    pub fn full_adder(state1: &Rc<Gate>, state2: &Rc<Gate>, carry: &Rc<Gate>) -> Ops {
        
        let out_xor = Rc::new(Gate::xor(state1.clone(), state2.clone()));
        let out_and = Rc::new(Gate::and(state1.clone(), state2.clone()));

        let out_xor2 = Rc::new(Gate::xor(out_xor.clone(), carry.clone()));
        let out_and2 = Rc::new(Gate::and(out_xor.clone(), carry.clone()));

        let out_or = Rc::new(Gate::Or(out_and.clone(), out_and2.clone()));

        Ops::FullAdder(
            out_xor2,
            out_or,
        )

    }

    pub fn adder_8bit(num1: &Rc<UnsignedInteger8>, num2: &Rc<UnsignedInteger8>) -> Ops {
        
        let mut result = Vec::with_capacity(8);
        let mut carry = Rc::new(Gate::constant(false));

        for (bit1, bit2) in num1.value.iter().rev().zip(num2.value.iter().rev()) {

            let fa = Ops::full_adder(&bit1.clone(), &bit2.clone(), &carry.clone());
            
            if let Ops::FullAdder(sum, carry_in) = &fa {
                result.push(sum.clone());
                carry = carry_in.clone();
            }
            else {
                panic!("Did not get a FullAdder returned from func full_adder");
            }

        }
        
        result.reverse();
        let array: [Rc<Gate>; 8] = result
            .try_into()
            .expect("UnsignedInteger8 did not return 8 Gates arrays");

        Ops::Adder8Bit(
            Rc::new(UnsignedInteger8::new(array)),
            carry
        )

    }

}
