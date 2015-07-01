mod galois_field_int;
use std::ops;

pub struct Polynomial {
    data: Vec<u8>,
}

impl Polynomial {
    fn new(src_data: Vec<u8>) -> Polynomial {
        Polynomial {data: src_data}
    }
}

impl ops::Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) ->Polynomial {
        Polynomial{data: other.data}
    }
}
