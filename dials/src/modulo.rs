mod r#impl;

#[derive(Debug, Clone, Copy)]
pub struct Modulo<const N: usize, T>(T);
//
// pub trait ToNumber {
//     fn to_number(self) -> usize;
// }
//
// impl<const N: usize, T: From<usize> + Eq + PartialEq> Modulo<N, T> {
//     fn increment(&mut self) -> &mut Self {
//         self
//     }
// }

#[cfg(test)]
mod tests {
    use super::Modulo;

    #[test]
    fn can_construct() {
        let n = Modulo::<8, u8>::new(0);
        let n = n + Modulo::<8, u8>::new(1);
        assert_eq!(n.0, 1);

        let m = Modulo::<1000, u16>::new(1001_u16);
        assert_eq!(m.0, 1);

        // let o = Modulo::<256, u8>::new(1);
    }
}
