
use crate::logic::gates::GateRef;



/// Struct to describe the inputs for a single general adder for an ALU
pub struct AluGateInputs {
    pub state1: GateRef,
    pub state2: GateRef,
    pub enable1: GateRef,
    pub invert2: GateRef,
    pub opcode2: GateRef,
    pub opcode1: GateRef,
    pub c_in: GateRef
}
