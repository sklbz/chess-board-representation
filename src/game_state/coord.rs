// use std::ops::Add;
// macro_rules! impl_modular_add {
//     ($name:ident, $n:expr) => {
//         impl Add for $name {
//             type Output = Self;
//             fn add(self, rhs: Self) -> Self {
//                 unsafe { std::mem::transmute((self as u8 + rhs as u8 + 1) % $n) }
//             }
//         }
//     };
// }
//
// impl_modular_add!(Coordinate2, 2);
// impl_modular_add!(Coordinate3, 3);
// impl_modular_add!(Coordinate4, 4);
// impl_modular_add!(Coordinate5, 5);
// impl_modular_add!(Coordinate6, 6);
// impl_modular_add!(Coordinate7, 7);
// impl_modular_add!(Coordinate8, 8);
// impl_modular_add!(Coordinate9, 9);

#[repr(u8)]
pub enum Coordinate2 {
    Alpha,
    Beta,
}
#[repr(u8)]
pub enum Coordinate3 {
    Alpha,
    Beta,
    Gamma,
}
#[repr(u8)]
pub enum Coordinate4 {
    Alpha,
    Beta,
    Gamma,
    Delta,
}
#[repr(u8)]
pub enum Coordinate5 {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
}
#[repr(u8)]
pub enum Coordinate6 {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
}
#[repr(u8)]
pub enum Coordinate7 {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
}
#[repr(u8)]
pub enum Coordinate8 {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
    Theta,
}
#[repr(u8)]
pub enum Coordinate9 {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
    Zeta,
    Eta,
    Theta,
    Iota,
}
