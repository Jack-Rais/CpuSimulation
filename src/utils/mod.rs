
use std::rc::Rc;

use crate::logic::gates::implement::Gate;
use crate::logic::gates::constant;


pub fn u8_to_4bit(num: u8) -> [Rc<dyn Gate>; 4] {

    if num > 15 { panic!("Number impossible to convert to a 4bit int") }

    let mut result = std::array::from_fn(|i| {
        let x: Rc<dyn Gate> = Rc::new(constant((num & (1 << i)) != 0)); x
    });

    result.reverse();

    result

}


pub fn bit4_to_u8(num: &[Rc<dyn Gate>; 4]) -> u8 {

    let mut result = 0u8;
    for (n, x) in num.iter().rev().enumerate() {
        result += 2u8.pow(n as u32) * (x.eval() as u8);
    }
    
    result
}
