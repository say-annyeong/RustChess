#[macro_export]
macro_rules! impl_add_sub_mul {
    ($(($t:ident, $field_name:ident)),*) => {
        $(
            impl<const D: usize> $t<D> {
                pub const fn overflowing_add(self, rhs: Self) -> (Self, [bool; D]) {
                    let mut $field_name = [0; D];
                    let mut overflow = [false; D];
                    let mut i = 0;
                    while i < D {
                        let (result, of) = self.$field_name[i].overflowing_add(rhs.$field_name[i]);
                        $field_name[i] = result;
                        overflow[i] = of;
                        i += 1;
                    }
                    (Self { $field_name }, overflow)
                }

                pub const fn overflowing_sub(self, rhs: Self) -> (Self, [bool; D]) {
                    let mut $field_name = [0; D];
                    let mut overflow = [false; D];
                    let mut i = 0;
                    while i < D {
                        let (result, of) = self.$field_name[i].overflowing_sub(rhs.$field_name[i]);
                        $field_name[i] = result;
                        overflow[i] = of;
                        i += 1;
                    }
                    (Self { $field_name }, overflow)
                }

                pub const fn overflowing_mul(self, rhs: Self) -> (Self, [bool; D]) {
                    let mut $field_name = [0; D];
                    let mut overflow = [false; D];
                    let mut i = 0;
                    while i < D {
                        let (result, of) = self.$field_name[i].overflowing_mul(rhs.$field_name[i]);
                        $field_name[i] = result;
                        overflow[i] = of;
                        i += 1;
                    }
                    (Self { $field_name }, overflow)
                }

                pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].unchecked_add(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    (Self { $field_name })
                }

                pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].unchecked_sub(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].unchecked_mul(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                    let overflow = self.overflowing_add(rhs).1;
                    let mut i = 0;
                    let mut is_overflow = false;
                    while i < D {
                        if overflow[i] {
                            is_overflow = true;
                            break;
                        }
                        i += 1;
                    }
                    if is_overflow {
                        None
                    } else {
                        // this safe
                        unsafe { Some(self.unchecked_add(rhs)) }
                    }
                }

                pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                    let overflow = self.overflowing_sub(rhs).1;
                    let mut i = 0;
                    let mut is_overflow = false;
                    while i < D {
                        if overflow[i] {
                            is_overflow = true;
                            break;
                        }
                        i += 1;
                    }
                    if is_overflow {
                        None
                    } else {
                        // this safe
                        unsafe { Some(self.unchecked_sub(rhs)) }
                    }
                }

                pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
                    let overflow = self.overflowing_mul(rhs).1;
                    let mut i = 0;
                    let mut is_overflow = false;
                    while i < D {
                        if overflow[i] {
                            is_overflow = true;
                            break;
                        }
                        i += 1;
                    }
                    if is_overflow {
                        None
                    } else {
                        // this safe
                        unsafe { Some(self.unchecked_mul(rhs)) }
                    }
                }

                pub const fn wrapping_add(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].wrapping_add(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const fn wrapping_sub(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].wrapping_sub(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const fn wrapping_mul(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].wrapping_mul(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const fn saturating_add(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].saturating_add(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const fn saturating_sub(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].saturating_sub(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

                pub const fn saturating_mul(self, rhs: Self) -> Self {
                    let mut $field_name = [0; D];
                    let mut i = 0;
                    while i < D {
                        let result = self.$field_name[i].saturating_mul(rhs.$field_name[i]);
                        $field_name[i] = result;
                        i += 1;
                    }
                    Self { $field_name }
                }

            }
        )*
    };
}

#[macro_export]
macro_rules! impl_ops_add_sub_mul_assign {
    ($(($t:ident, $field_name:ident)),*) => {
        use std::ops::{Add, AddAssign, Sub, SubAssign, MulAssign, Mul};
        $(
            impl<const D: usize> Add<$t<D>> for $t<D> {
                type Output = Self;

                #[inline(always)]
                fn add(self, rhs: Self) -> Self {
                    self.wrapping_add(rhs)
                }
            }

            impl<const D: usize> AddAssign<$t<D>> for $t<D> {
                #[inline(always)]
                fn add_assign(&mut self, rhs: Self) {
                    *self = self.wrapping_add(rhs);
                }
            }

            impl<const D: usize> Sub<$t<D>> for $t<D> {
                type Output = Self;

                #[inline(always)]
                fn sub(self, rhs: Self) -> Self {
                    self.wrapping_sub(rhs)
                }
            }

            impl<const D: usize> SubAssign<$t<D>> for $t<D> {
                #[inline(always)]
                fn sub_assign(&mut self, rhs: Self) {
                    *self = self.wrapping_sub(rhs);
                }
            }

            impl<const D: usize> Mul<$t<D>> for $t<D> {
                type Output = Self;

                #[inline(always)]
                fn mul(self, rhs: Self) -> Self {
                    self.wrapping_mul(rhs)
                }
            }

            impl<const D: usize> MulAssign<$t<D>> for $t<D> {
                #[inline(always)]
                fn mul_assign(&mut self, rhs: Self) {
                    *self = self.wrapping_mul(rhs);
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_try_from {
    ($(($t:ident, $field_name:ident, $from_type:ty)),*) => {
        $(
            impl<const D: usize> From<[$from_type; D]> for $t<D> {
                #[inline(always)]
                fn from(value: [$from_type; D]) -> Self {
                    Self { $field_name: value }
                }
            }

            impl<const D: usize> TryFrom<Vec<$from_type>> for $t<D> {
                type Error = Vec<$from_type>;

                #[inline(always)]
                fn try_from(value: Vec<$from_type>) -> Result<Self, Vec<$from_type>> {
                    match value.try_into() {
                        Ok($field_name) => Ok(Self { $field_name }),
                        Err(e) => Err(e),
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_ops_refs {
    ($(($t:ident, $field_name:ident, $ref_type:ty)),*) => {
        use std::ops::{Deref, DerefMut};
        $(
            impl<const D: usize> AsRef<[$ref_type; D]> for $t<D> {
                #[inline(always)]
                fn as_ref(&self) -> &[$ref_type; D] {
                    &self.$field_name
                }
            }

            impl<const D: usize> AsMut<[$ref_type; D]> for $t<D> {
                #[inline(always)]
                fn as_mut(&mut self) -> &mut [$ref_type; D] {
                    &mut self.$field_name
                }
            }

            impl<const D: usize> Deref for $t<D> {
                type Target = [$ref_type; D];

                #[inline(always)]
                fn deref(&self) -> &[$ref_type; D] {
                    &self.$field_name
                }
            }

            impl<const D: usize> DerefMut for $t<D> {
                #[inline(always)]
                fn deref_mut(&mut self) -> &mut [$ref_type; D] {
                    &mut self.$field_name
                }
            }
        )*
    }
}

#[macro_export]
macro_rules! impl_try_from_iterator {
    ($(($t:ident, $field_name:ident, $item_type:ty)),*) => {
        $(
            impl<const D: usize> $t<D> {
                pub fn try_from_iter<T: IntoIterator<Item = $item_type>>(iter: T) -> Option<Self> {
                    let vector: Vec<_> = iter.into_iter().collect();
                    match vector.try_into() {
                        Ok($field_name) => Some(Self { $field_name }),
                        Err(_) => None
                    }
                }
            }
        )*
    }
}