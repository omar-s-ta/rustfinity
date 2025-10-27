use core::f64;

pub fn data_types() -> (u8, f64, bool, char) {
    let a: u8 = 42;
    let b: f64 = f64::consts::PI;
    let c: bool = false;
    let d: char = 'a';
    (a, b, c, d)
}
