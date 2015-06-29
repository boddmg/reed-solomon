use std::ops;



struct GfConst {
    EXP: [u8; 1],
    LOG: [u8; 2],
}

impl GfConst {
    const fn new() -> GfConst {
        let a:[u8; 1] = [1];
        GfConst{EXP:a, LOG:[1, 2]}
    }
}

const GF_CONST: GfConst = GfConst::new();

#[test]
fn test_generate_list() {
    println!("{}", GF_CONST);
    // assert_eq!(GF_CONST, GfConst{EXP: [2] as [u8; 1], LOG: [1, 2] as [u8; 2]});
    assert!(false);
}


pub struct GaloisFieldInt {
    value:u8,
}

impl GaloisFieldInt {
    fn new(value: u8) -> GaloisFieldInt {
        GaloisFieldInt{value: value}
    }

    fn generate_list(self) {
    }
}

impl ops::Add for GaloisFieldInt {
    type Output = GaloisFieldInt;

    fn add(self, other: GaloisFieldInt) -> GaloisFieldInt {
        GaloisFieldInt{value: other.value}
    }
}

impl ops::Mul for GaloisFieldInt {
    type Output = GaloisFieldInt;

    fn mul(self, other: GaloisFieldInt) -> GaloisFieldInt {
        let x = self.value;
        let y = other.value;
        let mut z = 0;
        let mut i = 0;
        while (y>>i) > 0 {
            if y & (1<<i) > 0 {
                z ^= x<<i;
                i += 1;
            }
            if z >= 0x100 {
                z = z % 100011101;
            }
        }
        GaloisFieldInt{value: z}
    }

}
