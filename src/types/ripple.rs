use crate::logic::gates::GateRef;



pub struct GeneralSingleInputs {
    pub state1: GateRef,
    pub state2: GateRef
}

pub struct GeneralSingleCarryInputs {
    pub state1: GateRef,
    pub state2: GateRef,
    pub carry_in: GateRef
}

pub struct GeneralSingleCarryOutputs {
    pub out_sum: GateRef,
    pub carry_out: GateRef
}

pub struct General4BitCarryInputs<'a> {
    pub inp1: &'a [GateRef; 4],
    pub inp2: &'a [GateRef; 4],
    pub carry_in: GateRef,
}

pub struct General8BitInputs<'a> {
    pub num1: &'a [GateRef; 8],
    pub num2: &'a [GateRef; 8],
}

pub struct General8BitCarryOutputs {
    pub out_sum: [GateRef; 8],
    pub carry_out: GateRef
}



