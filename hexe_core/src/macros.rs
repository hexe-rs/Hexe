macro_rules! forward_bit_ops_impl {
    ($t:ident => $($t1:ident $f1:ident $t2:ident $f2:ident)+) => {
        $(impl<T: Into<$t>> ::core::ops::$t1<T> for $t {
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
        })+
    }
}

macro_rules! impl_set_ops {
    ($($t:ident)+) => {
        $(forward_bit_ops_impl! {
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

        /// Set operations.
        impl $t {
            /// Returns whether `self` contains `other`.
            #[inline]
            pub fn contains<T: Into<Self>>(&self, other: T) -> bool {
                let other = other.into().0;
                self.0 & other == other
            }

            /// Returns the number of bits set in `self`.
            #[inline]
            pub fn len(&self) -> usize {
                self.0.count_ones() as usize
            }

            /// Returns whether `self` is empty.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.0 == 0
            }
        })+
    }
}

// Allows for chaining `|`, `&`, and `^` without calling `T::from`
macro_rules! impl_composition_ops {
    ($u:ty => $($t:ty)+) => {
        $(impl<T: Into<$u>> ::core::ops::BitOr<T> for $t {
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
        })*
    }
}
