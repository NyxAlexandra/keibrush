macro_rules! impl_add {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_add! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };
    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Add<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Add<U>,
        {
            type Output = $Self<T::Output>;

            fn add(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.add(rhs.$field_),)*
                }
            }
        }

        impl<T, U> ::std::ops::AddAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::AddAssign<U>,
        {
            fn add_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.add_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_sub {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_sub! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };

    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Sub<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Sub<U>,
        {
            type Output = $Self<T::Output>;

            fn sub(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.sub(rhs.$field_),)*
                }
            }
        }

        impl<T, U> ::std::ops::SubAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::SubAssign<U>,
        {
            fn sub_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.sub_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_mul {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_mul! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };

    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Mul<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Mul<U>,
        {
            type Output = $Self<T::Output>;

            fn mul(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.mul(rhs.$field_),)*
                }
            }
        }

        impl<T, U> ::std::ops::MulAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::MulAssign<U>,
        {
            fn mul_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.mul_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_div {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_div! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };

    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Div<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Div<U>,
        {
            type Output = $Self<T::Output>;

            fn div(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.div(rhs.$field_),)*
                }
            }
        }

        impl<T, U> ::std::ops::DivAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::DivAssign<U>,
        {
            fn div_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.div_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_rem {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_rem! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };

    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Rem<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Rem<U>,
        {
            type Output = $Self<T::Output>;

            fn rem(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.rem(rhs.$field_)),*
                }
            }
        }

        impl<T, U> ::std::ops::RemAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::RemAssign<U>,
        {
            fn rem_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.rem_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_neg {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl<T> ::std::ops::Neg for $Self<T>
        where
            T: ::std::ops::Neg,
        {
            type Output = $Self<T::Output>;

            fn neg(self) -> Self::Output {
                $Self {
                    $($field: self.$field.neg()),*
                }
            }
        }
    };
}

macro_rules! impl_bitand {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_bitand! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };
    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::BitAnd<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::BitAnd<U>,
        {
            type Output = $Self<T::Output>;

            fn bitand(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.bitand(rhs.$field_)),*
                }
            }
        }

        impl<T, U> ::std::ops::BitAndAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::BitAndAssign<U>,
        {
            fn bitand_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.bitand_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_bitor {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_bitor! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };
    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::BitOr<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::BitOr<U>,
        {
            type Output = $Self<T::Output>;

            fn bitor(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.bitor(rhs.$field_)),*
                }
            }
        }

        impl<T, U> ::std::ops::BitOrAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::BitOrAssign<U>,
        {
            fn bitor_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.bitor_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_bitxor {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_bitxor! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };
    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::BitXor<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::BitXor<U>,
        {
            type Output = $Self<T::Output>;

            fn bitxor(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.bitxor(rhs.$field_)),*
                }
            }
        }

        impl<T, U> ::std::ops::BitXorAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::BitXorAssign<U>,
        {
            fn bitxor_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.bitxor_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_shl {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_shl! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };
    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Shl<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Shl<U>,
        {
            type Output = $Self<T::Output>;

            fn shl(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.shl(rhs.$field_)),*
                }
            }
        }

        impl<T, U> ::std::ops::ShlAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::ShlAssign<U>,
        {
            fn shl_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.shl_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_shr {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl_shr! {
            $Self -> $Self {
                $($field -> $field),*
            }
        }
    };
    (
        $Self:ident -> $Rhs:ident {
            $($field:ident -> $field_:ident),*
        }
    ) => {
        impl<T, U> ::std::ops::Shr<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::Shr<U>,
        {
            type Output = $Self<T::Output>;

            fn shr(self, rhs: $Rhs<U>) -> Self::Output {
                $Self {
                    $($field: self.$field.shr(rhs.$field_)),*
                }
            }
        }

        impl<T, U> ::std::ops::ShrAssign<$Rhs<U>> for $Self<T>
        where
            T: ::std::ops::ShrAssign<U>,
        {
            fn shr_assign(&mut self, rhs: $Rhs<U>) {
                $(
                    self.$field.shr_assign(rhs.$field_);
                )*
            }
        }
    };
}

macro_rules! impl_not {
    (
        $Self:ident {
            $($field:ident),*
        }
    ) => {
        impl<T> ::std::ops::Not for $Self<T>
        where
            T: ::std::ops::Not,
        {
            type Output = $Self<T::Output>;

            fn not(self) -> Self::Output {
                $Self {
                    $($field: self.$field.not()),*
                }
            }
        }
    };
}

pub(crate) use {
    impl_add,
    impl_bitand,
    impl_bitor,
    impl_bitxor,
    impl_div,
    impl_mul,
    impl_neg,
    impl_not,
    impl_rem,
    impl_shl,
    impl_shr,
    impl_sub,
};
