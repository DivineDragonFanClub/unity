#![feature(ptr_sub_ptr)]

pub use unity_macro::*;

#[doc(hidden)]
pub mod macro_context;
pub mod cppvector;
pub mod engine;
/// The core of this library. Contains the structures to interface with Il2Cpp and its internals.
pub mod il2cpp;
pub mod system;

extern crate memoffset;

use thiserror::Error;

/// A specialized Result type for Il2Cpp methods.
pub type Il2CppResult<T> = core::result::Result<T, Il2CppError>;

#[derive(Debug, Error)]
pub enum Il2CppError {
    #[error("could not find the class `{0}`")]
    MissingClass(String),
    #[error("could not find the class for the Il2CppType")]
    MissingClassForType,
    #[error("could not find the method")]
    MissingMethod,
    #[error("could not instantiate the class `{0}`")]
    FailedInstantiation(String),
    #[error("could not instantiate the array")]
    FailedArrayInstantiation,
    #[error("could not invoke the method")]
    FailedMethodInvocation,
    #[error("could not get a ReflectionType for the type")]
    FailedReflectionQuerying,
}

pub mod prelude {
    //! The Il2Cpp prelude.
    //!
    //! A collection of traits and types you’ll likely need when modding games built with Il2Cpp.
    //!
    //! ```
    //! # #![allow(unused_imports)]
    //! use unity::prelude::*;
    //! ```

    pub use crate::{
        get_generic_class,
        il2cpp,
        Il2CppResult,
        Il2CppError,
        il2cpp::{
            class::{Il2CppClass, Il2CppClassData},
            method::{MethodInfo, OptionalMethod},
            object::{Il2CppArray, Il2CppObject, ArrayInstantiator},
        },
        system::Il2CppString,
    };
}
