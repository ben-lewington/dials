pub use dials_macros::spec;

pub struct Modulo<const N: usize, T>(T);

macro_rules! impl_for {
    ($($n:ty),*) => {
$(
    impl<const N: usize> Modulo<N, $n>
    {
        const _CHECK: u128 = <$n>::MAX as u128 - N as u128;
        pub fn new(value: $n) -> Self {
            let _ = Self::_CHECK;
            return Self(value % N as $n);
        }
    }

    impl<const N: usize> std::ops::Add for Modulo<N, $n> {
        type Output = Modulo<N, $n>;

        fn add(self, rhs: Self) -> Self::Output {
            // SOUNDNESS: l + r could overflow in $n, which would give an incorrect value for the
            //            modulus.
            //
            // i.e. we require:
            //
            //     ((l + r) mod $n::MAX) mod N == (l + r) mod N | ∀ l, r ∈ ℕ,  0 <= l, r < N.
            //
            // Consider the following:
            //     (l + r) mod                       |
            //  == (l + (N - N) + r) mod N           | N - N == 0, x + 0 == 0 + x == x ∀x ∈ ℤ
            //  == ((l + N) + (r - N)) mod N         | rearrange
            //  == (l + N) mod N - (N - r) mod N     | mod is a homomorphism
            //  == l mod N + N mod N - (N - r) mod N | as l < N, l % N == l
            //  == l - (N - r) mod N                 | N % N == 0
            //  we can express the modulo add as two modulo subtractions,
            //  which are sound.
            //
            //  TODO: think through this and optimize
            Self(self.0) - Self(N as $n - rhs.0)
        }
    }

    impl<const N: usize> std::ops::Sub for Modulo<N, $n> {
        type Output = Modulo<N, $n>;

        fn sub(self, rhs: Self) -> Self::Output {
            // SOUNDNESS:
            // we know:
            //     l, r < N
            // =>
            //     0 <= |l - r| < N
            // so it suffices to ensure the subtraction does not underflow in $n.
            //
            if self.0 >= rhs.0 {
                return Self(self.0 - rhs.0);
            }
            // here, we know l < r, therefore r - l does not underflow
            // we know:
            //     -x % N == (N - x) % N
            // for each x, N, thus we have
            //     (l - r) % N == -(r - l) % N
            //                 == (N - (r - l)) % N
            Self(N as $n - (rhs.0 - self.0))
        }
    }
)*
    };
}

impl_for! {u8, u16, u32, u64, u128, usize}

#[cfg(test)]
mod tests {
    use crate::Modulo;

    #[test]
    fn can_construct() {
        let n = Modulo::<8, u8>::new(0);
        let n = n + Modulo::<8, u8>::new(1);
        assert_eq!(n.0, 1);
    }
}
