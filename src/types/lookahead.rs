use crate::logic::gates::GateRef;


pub struct GPSingleOutputs {
    pub out: GateRef,
    pub propagate: GateRef,
    pub generate: GateRef
}

pub struct GP3BitOutputs {
    pub out: [GateRef; 3],
    pub propagate: GateRef,
    pub generate: GateRef
}

pub struct GP4BitOutputs {
    pub out: [GateRef; 4],
    pub propagate: GateRef,
    pub generate: GateRef
}



