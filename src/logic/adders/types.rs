

use crate::logic::gates::GateRef;


pub struct CarryLookaheadUnitInputs<'a> {
    pub props: &'a [GateRef; 4],
    pub gens: &'a [GateRef; 4],
    pub carry_in: GateRef,
}


pub struct CarryLookaheadUnitGPOutputs {
    pub carry_outs: [GateRef; 3],
    pub propagate: GateRef,
    pub generate: GateRef,
}



pub struct Adder4BitInputs<'a> {
    pub num1: &'a[GateRef; 4],
    pub num2: &'a[GateRef; 4],
    pub carry_in: GateRef,
}

pub struct Adder4BitGPOutputs {
    pub sum: [GateRef; 4],
    pub propagate: GateRef,
    pub generate: GateRef,
}
