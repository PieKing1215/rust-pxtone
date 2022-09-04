#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::cast_sign_loss)]
#![warn(clippy::result_unit_err)]

mod pxtone;
pub use self::pxtone::*;
