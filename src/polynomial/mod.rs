mod galois_field_int;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

pub struct Polynomial {
    data: Vec<u8>,
}

impl Polynomial {
    fn new(src_data: Vec<u8>) -> Polynomial {
        Polynomial {data: src_data}
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) ->Polynomial {
        Polynomial{data: other.data}
    }
}
