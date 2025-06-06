pub mod effect;
pub mod scoreboard;
pub mod tellraw;
use std::fmt::Display;

pub use effect::*;
pub use scoreboard::*;
pub use tellraw::*;

pub trait Command: Display {}

#[macro_export]
macro_rules! factory {
    ($struct:ident => $function:ident) => {
        pub struct $struct;

        pub fn $function() -> $struct {
            $struct {}
        }
    };
}

#[macro_export]
macro_rules! subcommands {
    ($struct:ident {
        $(unit {
            $($unit_c:ident() => $unit_s:ident),*
        };)?
        $(new {
            $($new_c:ident($($new_arg:ident: $new_type:ty),*) => $new_s:ident),*
        };)?
        $(new with $generic:ident {
            $($gen_c:ident($($gen_arg:ident: $gen_type:ty),*) => $gen_s:ident),*
        };)?
    }) => {
        impl $struct {
            $($(
                pub fn $unit_c(self) -> $unit_s {
                    $unit_s {}
                }
            )*)*
            $($(
                pub fn $new_c(self, $($new_arg: $new_type),*) -> $new_s {
                    $new_s::new($($new_arg),*)
                }
            )*)*
            $($(
                pub fn $gen_c<T: $generic>(self, $($gen_arg: $gen_type),*) -> $gen_s<T> {
                    $gen_s::new($($gen_arg),*)
                }
            )*)*
        }
    };
}

#[macro_export]
macro_rules! arguments {
    ($struct:ident $(with $g_name:ident: $generic:ident)? => $as_string:literal {
        $(required {
            $($(#[$req_attr:meta])* $req_arg_name:ident: $req_arg_type:ty),+
        };)?
        $(optional {
            $($(#[$opt_attr:meta])* $opt_arg_name:ident: $opt_arg_type:ty),+
        };)?
    }) => {
        #[derive(::derive_new::new)]
        pub struct $struct$(<$g_name: $generic>)? {
            $($(
                $(#[$req_attr])*
                $req_arg_name: $req_arg_type,
            )+)?
            $($(
                $(#[$opt_attr])*
                #[new(default)]
                $opt_arg_name: Option<$opt_arg_type>,
            )+)?
        }

        impl$(<$g_name: $generic>)? ::std::fmt::Display for $struct$(<$g_name>)? {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", $as_string)?;
                $($(
                    write!(f, " {}", self.$req_arg_name)?;
                )+)?
                $($(
                    if let Some($opt_arg_name) = &self.$opt_arg_name {
                        write!(f, " {}", $opt_arg_name)?;
                    }
                )+)?
                Ok(())
            }
        }

        impl$(<$g_name: $generic>)? $crate::commands::Command for $struct$(<$g_name>)? {}
    };
}
