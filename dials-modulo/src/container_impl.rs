use super::Mod;

// Some type we might be using to model modular arithmetic.
pub trait NatContainer: Sized {
    const MIN: usize;
    const MAX: usize;

    fn repr(self) -> usize;

    fn repr_wide(self) -> u128;

    // fn try_cast<N: NatContainer>(self) -> Result<N, usize>;
}

nat::impl_for! {u8, u16, u32, u64, u128, usize}

mod nat {

macro_rules! impl_for {
    ($($n:ty),*) => {
        $(

impl NatContainer for $n {
    const MIN: usize = <$n>::MIN as usize;
    const MAX: usize = <$n>::MAX as usize;

    fn repr(self) -> usize { self as usize }

    fn repr_wide(self) -> u128 { self as u128 }
}

impl<const N: usize> Mod<N, $n> {
    const _CHECK: u128 = <$n>::MAX as u128 - N as u128;
    pub fn new(value: $n) -> Self {
        let _ = Self::_CHECK;
        let value = value as $n;
        if value >= N as $n {
            return Self(value % N as $n)
        }
        Self(value)
    }
}

impl<const N: usize> std::ops::Add for Mod<N, $n> {
    type Output = Mod<N, $n>;

    fn add(self, rhs: Self) -> Self::Output {
        // SOUNDNESS:
        //
        // l + r could overflow in $n, which would give an incorrect value for the modulus.
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
        //
        //  Q.E.D we can express the modulo add as two modulo subtractions,
        //  which are sound.
        //
        //  TODO: think through this and optimize
        Self(self.0) - Self(N as $n - rhs.0)
    }
}

impl<const N: usize> std::ops::Sub for Mod<N, $n> {
    type Output = Mod<N, $n>;

    fn sub(self, rhs: Self) -> Self::Output {
        // SOUNDNESS:
        //
        // we know:
        //
        //     l, r < N
        // =>
        //     0 <= |l - r| < N
        //
        // so it suffices to ensure the subtraction does not underflow in $n.
        //
        // consider the case l < r (i.e. the subtraction would underflow).
        //
        // we know that  r - l does not underflow.
        //
        // we also know:
        //     -x % N == (N - x) % N
        // thus we have
        //     (l - r) % N == -(r - l) % N
        //                 == (N - (r - l)) % N
        //
        if self.0 < rhs.0 {
            return Self(N as $n - (rhs.0 - self.0));
        }
        return Self(self.0 - rhs.0)
    }
}
        )*
    };
}

pub(crate) use impl_for;

}
