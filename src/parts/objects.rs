

pub struct UnsignedInteger8 {
    value: [bool; 8]
}

impl UnsignedInteger8 {

    pub fn new() -> Self {
        Self { value: [true; 8] }
    }

}
