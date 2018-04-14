macro_rules! forward_bit_ops_impl {
    ($t:ident => $($t1:ident $f1:ident $t2:ident $f2:ident)+) => { $(
        impl<T: Into<$t>> ::core::ops::$t1<T> for $t {
            type Output = Self;

            #[inline]
            fn $f1(self, other: T) -> Self {
                $t((self.0).$f1(other.into().0))
            }
        }

        impl<T: Into<$t>> ::core::ops::$t2<T> for $t {
            #[inline]
            fn $f2(&mut self, other: T) {
                (self.0).$f2(other.into().0)
            }
        }
    )+ }
}

macro_rules! squares {
    ($($s:ident),+) => {
        $(1 << ::square::Square::$s as u64)|+
    }
}

macro_rules! impl_rand {
    ($s:ty => $($t:ty),+) => { $(
        #[cfg(any(test, feature = "rand"))]
        impl ::rand::Rand for $t {
            #[inline]
            fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
                rng.gen::<$s>().into()
            }
        }
    )+ }
}

macro_rules! impl_bit_set {
    ($($t:ident $full:expr => $x:ident);+ $(;)*) => { $(
        forward_bit_ops_impl! {
            $t =>
            BitAnd bitand BitAndAssign bitand_assign
            BitXor bitxor BitXorAssign bitxor_assign
            BitOr  bitor  BitOrAssign  bitor_assign
        }

        impl<T: Into<$t>> ::core::ops::Sub<T> for $t {
            type Output = Self;

            #[inline]
            fn sub(self, other: T) -> Self { $t(self.0 & !other.into().0) }
        }

        impl<T: Into<$t>> ::core::ops::SubAssign<T> for $t {
            #[inline]
            fn sub_assign(&mut self, other: T) { self.0 &= !other.into().0 }
        }

        impl ::core::ops::Not for $t {
            type Output = Self;

            #[inline]
            fn not(self) -> Self { $t(!self.0 & $full) }
        }

        impl<A: Into<$t>> ::core::iter::FromIterator<A> for $t {
            #[inline]
            fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
                iter.into_iter().fold(Self::EMPTY, ::core::ops::BitOr::bitor)
            }
        }

        impl<A: Into<$t>> Extend<A> for $t {
            #[inline]
            fn extend<T: IntoIterator<Item=A>>(&mut self, iter: T) {
                *self |= iter.into_iter().collect::<$t>();
            }
        }

        impl Iterator for $t {
            type Item = $x;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> { self.pop_lsb() }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len();
                (len, Some(len))
            }

            #[inline]
            fn count(self) -> usize { self.len() }

            #[inline]
            fn last(self) -> Option<Self::Item> { self.msb() }
        }

        impl DoubleEndedIterator for $t {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> { self.pop_msb() }
        }

        impl ExactSizeIterator for $t {
            #[inline]
            fn len(&self) -> usize { $t::len(self) }
        }

        impl<T: Into<$t>> ::misc::Contained<$t> for T {
            #[inline]
            fn contained_in(self, other: $t) -> bool {
                let this = self.into().0;
                other.0 & this == this
            }
        }

        /// Bit set operations.
        impl $t {
            /// An instance with all bits set to 1.
            pub const FULL: $t = $t($full);

            /// An instance with all bits set to 0.
            pub const EMPTY: $t = $t(0);

            /// Returns whether `self` contains `other`.
            #[inline]
            pub fn contains<T: ::misc::Contained<Self>>(self, other: T) -> bool {
                other.contained_in(self)
            }

            /// Returns whether `self` intersects with `other`.
            #[inline]
            pub fn intersects<T: Into<Self>>(self, other: T) -> bool {
                !(self & other).is_empty()
            }

            /// Returns the number of bits set in `self`.
            ///
            /// Set the following when compiling to use the much faster hardware
            /// implementation **(`x86` and `x86_64` only)**:
            ///
            /// ```sh
            /// RUSTFLAGS="-C target-feature +popcnt"
            /// ```
            #[inline]
            pub fn len(&self) -> usize {
                self.0.count_ones() as usize
            }

            /// Returns whether `self` is empty.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0 == 0
            }

            /// Returns whether `self` has multiple bits set.
            #[inline]
            pub fn has_multiple(&self) -> bool {
                self.0 & self.0.wrapping_sub(1) != 0
            }

            /// Converts `self` into its single bit.
            #[inline]
            pub fn into_bit(mut self) -> Option<$x> {
                let bit = self.pop_lsb();
                if self.is_empty() { bit } else { None }
            }

            /// Returns the least significant bit of `self`.
            #[inline]
            pub fn lsb(&self) -> Option<$x> {
                if self.is_empty() { None } else {
                    unsafe { Some(self.lsb_unchecked()) }
                }
            }

            /// Returns the most significant bit of `self`.
            #[inline]
            pub fn msb(&self) -> Option<$x> {
                if self.is_empty() { None } else {
                    unsafe { Some(self.msb_unchecked()) }
                }
            }

            /// Returns the least significant bit of `self` without checking
            /// whether `self` is empty.
            #[inline]
            pub unsafe fn lsb_unchecked(&self) -> $x {
                use uncon::*;
                self.0.trailing_zeros().into_unchecked()
            }

            /// Returns the least significant bit of `self` without checking
            /// whether `self` is empty.
            #[inline]
            pub unsafe fn msb_unchecked(&self) -> $x {
                use core::mem;
                use uncon::*;
                let bits = mem::size_of::<Self>() * 8 - 1;
                (bits ^ self.0.leading_zeros() as usize).into_unchecked()
            }

            /// Removes the least significant bit from `self`.
            #[inline]
            pub fn remove_lsb(&mut self) {
                self.0 &= self.0.wrapping_sub(1);
            }

            /// Removes the most significant bit from `self`.
            #[inline]
            pub fn remove_msb(&mut self) {
                self.pop_msb();
            }

            /// Removes the least significant bit from `self` and returns it.
            #[inline]
            pub fn pop_lsb(&mut self) -> Option<$x> {
                self.lsb().map(|x| {
                    self.remove_lsb();
                    x
                })
            }

            /// Removes the most significant bit from `self` and returns it.
            #[inline]
            pub fn pop_msb(&mut self) -> Option<$x> {
                self.msb().map(|x| {
                    self.0 ^= Self::from(x).0;
                    x
                })
            }
        }
    )+ }
}

// Allows for chaining `|`, `&`, and `^` without calling `T::from`
macro_rules! impl_composition_ops {
    ($u:ty => $($t:ty)+) => { $(
        impl<T: Into<$u>> ::core::ops::BitOr<T> for $t {
            type Output = $u;

            #[inline]
            fn bitor(self, other: T) -> $u {
                other.into().bitor(self)
            }
        }

        impl<T: Into<$u>> ::core::ops::BitAnd<T> for $t {
            type Output = $u;

            #[inline]
            fn bitand(self, other: T) -> $u {
                other.into().bitand(self)
            }
        }

        impl<T: Into<$u>> ::core::ops::BitXor<T> for $t {
            type Output = $u;

            #[inline]
            fn bitxor(self, other: T) -> $u {
                other.into().bitxor(self)
            }
        }
    )+ }
}

macro_rules! define_from_str_error {
    ($t:ty; #[$m:meta] $msg:expr) => {
        #[$m] #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct FromStrError(());

        impl fmt::Display for FromStrError {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                $msg.fmt(f)
            }
        }

        #[cfg(feature = "std")]
        impl ::std::error::Error for FromStrError {
            #[inline]
            fn description(&self) -> &str { $msg }
        }

        #[cfg(feature = "serde")]
        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                <&str>::deserialize(de)?.parse().map_err(|_| {
                    de::Error::custom($msg)
                })
            }
        }
    }
}
