//FIXME
//#[cfg(feature = "paste")]
#[macro_export]
macro_rules! impl_from_for_snafu_error {
    ($type:path, $error:ident, $base_error:ident) => {
        impl From<$type> for $base_error {
            fn from(error: $type) -> Self {
                Self::$error { source: error }
            }
        }
    };
    ($type:path, $error:ident) => {
        impl_from_for_snafu_error! { $type, $error, Error }
    };
    ($name:ident) => {
        ::paste::paste! {
            impl_from_for_snafu_error! { [<$name Error>], $name }
        }
    };
}
