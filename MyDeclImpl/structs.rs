/// `Struct::default()`: assigning user-defined values to fields directly.
/// 
/// # Principles
/// 
/// Text replacement, automatic function generation.
/// 
/// # Examples
/// 
/// ``` rust
/// use aoko::{struct_default, assert_eqs};
/// 
/// struct_default!(
///     #[derive(Debug)]
///     pub struct A<'a> {
///         foo: u8 = 233,
///         pub bar: &'a str = "abc",
///     }
///     struct B {}
///     struct C;
/// );
/// 
/// assert_eqs!(
///     233, A::default().foo;
///     "abc", A::default().bar;
///     "A { foo: 233, bar: \"abc\" }", format!("{:?}", A::default());
/// );
/// ```
#[macro_export]
macro_rules! struct_default {
    ($vis:vis struct $s_name:ident;) => {$vis struct $s_name;};

    ($(#[$attr:meta])* $vis:vis struct $name:ident $(<$($generic:tt),*>)? {
        $($field_vis:vis $field:ident: $type:ty = $val:expr),* $(,)?
    }
    $($tail:tt)*) => {
        $(#[$attr])*
        $vis struct $name $(<$($generic),*>)? {
            $($field_vis $field: $type),*
        }
        impl $(<$($generic),*>)? Default for $name $(<$($generic),*>)? {
            fn default() -> Self {
                $name {
                    $($field: $val),*
                }
            }
        }
        struct_default! {
            $($tail)*
        }
    };

    () => {}
}

/// `Struct::new(...)`: define parameters and assigning user-defined values to fields directly.
/// 
/// # Principles
/// 
/// Text replacement, automatic function generation.
/// 
/// # Examples
/// 
/// ``` rust
/// use aoko::{struct_new, assert_eqs};
/// 
/// struct_new!(
///     #[derive(Debug)]
///     pub struct A<'a, T>(pub foo: T,) where T: Copy, T: Ord {
///         pub bar: &'a str = "bar",
///     }
///     struct B {}
///     struct C;
/// );
/// 
/// let test = A::new("foo");
/// 
/// assert_eqs!(
///     "foo", test.foo;
///     "bar", test.bar;
///     format!("{:?}", test), "A { foo: \"foo\", bar: \"bar\" }";
/// );
/// ```
#[macro_export]
macro_rules! struct_new {
    ($vis:vis struct $s_name:ident;) => {$vis struct $s_name;};

    ($(#[$attr:meta])* $vis:vis struct $s_name:ident $(<$($generic:tt),*>)? $(($($p_vis:vis $p_name:ident: $p_type:ty),* $(,)?))? $(where $($id:tt: $limit:tt),*)? {
        $($field_vis:vis $field:ident: $type:ty = $val:expr),* $(,)?
    }
    $($tail:tt)*) => {
        $(#[$attr])*
        $vis struct $s_name $(<$($generic),*>)? $(where $($id: $limit),*)? {
            $($($p_vis $p_name: $p_type,)*)?
            $($field_vis $field: $type),*
        }
        impl $(<$($generic),*>)? $s_name $(<$($generic),*>)? $(where $($id: $limit),*)? {
            fn new($($($p_name: $p_type),*)?) -> Self {
                $s_name {
                    $($($p_name,)*)?
                    $($field: $val),*
                }
            }
        }
        struct_new! {
            $($tail)*
        }
    };

    () => {}
}
