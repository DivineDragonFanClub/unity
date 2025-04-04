pub use std::sync::LazyLock;
pub use crate::
    il2cpp::{
        class::{
            Il2CppClass,
            Il2CppClassData
        },
        object::Il2CppObjectMethods,
        method::MethodInfo
    }
;
pub use lazysimd::scan;