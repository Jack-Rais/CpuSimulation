
pub mod utils;
pub mod types;
pub mod logic;
pub mod cpu;

use utils::{bit4_to_u8, u8_to_4bit};
use crate::logic::gates::{constant, GateRef};
use crate::logic::adders::carry_lookahead::adder_4bit_cl;
use crate::types::ripple::General4BitCarryInputs;


fn main() {

    let x = u8_to_4bit(4);
    let y = u8_to_4bit(6);

    println!("x: {:?}", x.clone().map(|x| x.eval()));
    println!("y: {:?}", y.clone().map(|y| y.eval()));

    let z: GateRef = constant(false);

    let adder = adder_4bit_cl(General4BitCarryInputs {
        inp1: &x, 
        inp2: &y, 
        carry_in: z
    });
    

    println!(
        "{:?} - {} - {}",
        adder.out.clone().map(|x| x.eval()),
        adder.propagate.eval(),
        adder.generate.eval()
    );

    let res = bit4_to_u8(&adder.out);
    println!("{res}");

}
