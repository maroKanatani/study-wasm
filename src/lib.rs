extern crate tinymt;

use tinymt::tinymt32;

#[no_mangle]
pub fn rand() -> u32 {
    let param = tinymt32::Param {
        mat1: 0x8F7011EE,
        mat2: 0xFC78FF1F,
        tmat: 0x3793fdff,
    };
    let seed: u32 = 1;
    tinymt32::from_seed(param, seed).gen()
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

extern {
    fn date_now() -> f64;
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    unsafe {
        date_now()
    }
}