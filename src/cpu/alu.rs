
// ARITHMETIC LOGIC UNIT

// 0000 : ADD -> a + b
// 0001 : ADC -> a + b + 1 if last op. gen. a carry
// 0010 : SUB -> a - b
// 0011 : SBB -> a - b - 1 if last op. did not gen. a carry
// 0100 : ONC -> inverts bits of b
// 0101 : TWC -> inverts bits of b and adds 1
// 0110 : AND -> bitwise and on bits of a and b
// 0111 : OR  -> bitwise or on bits of a and b
// 1000 : XOR -> bitwise xor on bits of a and b
// 1001 : LSL -> logical left shift on b
// 1010 : LSR -> logical right shift on b
// 1011 : ASR -> arithmetic right shift on b
// 1100 : ROL -> rotate left on b
// 1101 : ROR -> rotate right on b
// 1110 : RCL -> rotate left on b through carry
// 1111 : RCR -> rotate right on b through carry


use std::rc::Rc;
use crate::logic::gates::Gate;

// Creates an ALU and outputs (out, propagate, generate)
// state1 -> A
// state2 -> B
// en_s1  -> enable A
// in_s2  -> invert B
// x1     -> OP code 1
// x2     -> OP code 2
// c_in   -> carry in

// (en) -> nand
// (en, x1) -> xor
pub fn init_alu(
    state1: Rc<Gate>,
    state2: Rc<Gate>,
    en_s1: Rc<Gate>,
    in_s2: Rc<Gate>,
    x1: Rc<Gate>,
    x2: Rc<Gate>,
    c_in: Rc<Gate>
) -> (Rc<Gate>, Rc<Gate>, Rc<Gate>) {
    
    let nxor = Rc::new(Gate::not(
        Rc::new(Gate::xor(
            in_s2.clone(),
            state2.clone()
        ))
    ));

    let nand = Rc::new(Gate::nand(
        en_s1.clone(),
        state1.clone()
    ));

    let and = Rc::new(Gate::and(
        Rc::new(Gate::and(
            nand.clone(),
            x1.clone()
        )), 
        nxor.clone()
    ));

    let nor = Rc::new(Gate::not(
        Rc::new(Gate::or(
            Rc::new(Gate::or(
                nand.clone(), 
                x2.clone()
            )),
            nxor.clone()
        ))
    ));

    let nor2 = Rc::new(Gate::not(
        Rc::new(Gate::or(
            and.clone(), 
            nor.clone()
        ))
    ));

    let xor = Rc::new(Gate::xor(
        c_in.clone(), 
        nor2.clone()
    ));

    (xor, nor2, nor)

}
