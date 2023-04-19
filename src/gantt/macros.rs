/// Setters,  macros::setters! { task: String => task }
#[macro_export]
macro_rules! setters {
    (@single $name:ident : $typ:ty => $transform:expr) => {
        #[allow(clippy::redundant_field_names)]
        #[allow(clippy::needless_update)]
        #[allow(missing_docs)]
        pub fn $name<P: ::std::convert::Into<$typ>>(self, $name: P) -> Self {
            let $name: $typ = $name.into();
            Self  {
                $name: $transform,
                ..self
            }
        }
    };
    // Terminal condition
    (@recurse) => {};
    // Recurse without transform
    (@recurse $name:ident : $typ:ty, $($tokens:tt)*) => {
        $crate::setters! { @recurse $name: $typ => $name, $($tokens)* }
    };
    // Recurse with transform
    (@recurse $name:ident : $typ:ty => $transform:expr, $($tokens:tt)*) => {
        $crate::setters! { @single $name : $typ => $transform }
        $crate::setters! { @recurse $($tokens)* }
    };
    ($($tokens:tt)*) => {
        $crate::setters! { @recurse $($tokens)* }
    }
}

pub(crate) use setters;
