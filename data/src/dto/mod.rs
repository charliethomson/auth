pub mod application;
pub mod error;
pub mod grant;
pub mod user;
pub mod user_grant;

#[macro_export]
macro_rules! impl_try_from_with {
    ($on:ident, $ns:tt, $with:ident, $error:ident, [$($field:ident,)*]) => {

        impl TryFrom<$crate::model::$ns::Model> for $on {
            type Error = $error;

            fn try_from(value: $crate::model::$ns::Model) -> Result<Self, Self::Error> {
                Self::$with(
                    $(
                        value.$field,
                    )+
                )
            }
        }

        impl TryFrom<$crate::model::$ns::ModelEx> for $on {
            type Error = $error;

            fn try_from(value: $crate::model::$ns::ModelEx) -> Result<Self, Self::Error> {
                Self::$with(
                    $(
                        value.$field,
                    )+
                )
            }
        }

    };
}
