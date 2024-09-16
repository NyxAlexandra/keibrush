/// Trait for types with a zero value.
pub trait Zero: Copy {
    /// The 0 value.
    const ZERO: Self;
}

/// Trait for types with a one value.
pub trait One: Copy {
    /// The 1 value.
    const ONE: Self;
}

/// Trait for types with negative one value.
pub trait NegOne: Copy {
    /// The -1 value.
    const NEG_ONE: Self;
}

/// Trait for types with a minimum value.
pub trait Min: Copy {
    /// The minimum value for this type.
    const MIN: Self;
}

/// Trait for types with a maximum value.
pub trait Max: Copy {
    /// The maximum value for this type.
    const MAX: Self;
}

/// Trait for types with an infinite value.
pub trait Infinity: Copy {
    /// The infinity value for this type.
    const INFINITY: Self;
}

/// Trait for types with trigonometric functions.
pub trait Trig: Copy {
    /// Computes the sine of `self`.
    fn sin(self) -> Self;

    /// Computes the cosine of `self`.
    fn cos(self) -> Self;

    /// Computes the tangent of `self`.
    fn tan(self) -> Self;

    /// Computes both the sine and cosine of `self`.
    fn sin_cos(self) -> (Self, Self);
}

macro_rules! impl_num {
    ($($ty:ident),*) => {
        $(
            impl Zero for $ty {
                const ZERO: Self = 0 as _;
            }

            impl One for $ty {
                const ONE: Self = 1 as _;
            }

            impl Min for $ty {
                const MIN: Self = <$ty>::MIN;
            }

            impl Max for $ty {
                const MAX: Self = <$ty>::MAX;
            }
        )*
    };
}

macro_rules! impl_neg {
    ($($ty:ident),*) => {
        $(
            impl_num!($ty);

            impl NegOne for $ty {
                const NEG_ONE: Self = -1 as _;
            }
        )*
    };
}

macro_rules! impl_float {
    ($($ty:ident),*) => {
        $(
            impl_neg!($ty);

            impl Infinity for $ty {
                const INFINITY: Self = <$ty>::INFINITY;
            }

            impl Trig for $ty {
                fn sin(self) -> Self {
                    $ty::sin(self)
                }

                fn cos(self) -> Self {
                    $ty::cos(self)
                }

                fn tan(self) -> Self {
                    $ty::tan(self)
                }

                fn sin_cos(self) -> (Self, Self) {
                    $ty::sin_cos(self)
                }
            }
        )*
    }
}

impl_num!(u8, u16, u32, u64, u128, usize);
impl_neg!(i8, i16, i32, i64, i128, isize);
impl_float!(f32, f64);
