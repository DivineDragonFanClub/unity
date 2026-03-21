#![allow(dead_code)]

use std::sync::LazyLock;

pub mod api;
pub mod assembly;
use assembly::*;
pub mod class;
use class::*;
pub mod object;
use object::*;
pub mod method;
use method::*;

use crate::{Il2CppResult, Il2CppError};
mod ffi;
pub mod generic;

pub fn method_from_name(name: impl AsRef<str>) -> *const u8 {
    let name = std::ffi::CString::new(name.as_ref()).unwrap();

    unsafe { method_name(name.as_ptr() as _) }
}

#[skyline::from_offset(0x491ff0)]
fn method_name(name: *const u8) -> *const u8;

#[repr(C)]
pub struct Il2CppDomain;

#[repr(C)]
pub union Il2CppTypeData {
    data: *const u8,
    class_index: i32,
    ty: &'static Il2CppType,
    array: *const u8, // &'static Il2CppArrayType
    generic_parameter_index: i32,
    generic_class: &'static Il2CppGenericClass,
}

#[repr(C)]
pub struct Il2CppType {
    pub data: Il2CppTypeData,
    bits: u32,
}

impl Il2CppType {
    pub fn get_object(ty: &Self) -> Il2CppResult<&'static mut Il2CppReflectionType> {
        unsafe { api::type_get_object(ty) }.ok_or(Il2CppError::FailedReflectionQuerying)
    }
}

pub fn instantiate_class<T: 'static>(class: &Il2CppClass) -> Il2CppResult<&'static mut T> {
    unsafe { api::object_new(class) }.ok_or(Il2CppError::FailedInstantiation(class.get_name()))
}

pub fn instantiate_class_by_name<T: 'static>(namespace: impl AsRef<str>, name: impl AsRef<str>) -> Il2CppResult<&'static mut T> {
    let class = class::Il2CppClass::from_name(namespace, name)?;
    instantiate_class(class)
}

pub fn il2cpp_init_scan() -> usize {
    static OFFSETS: LazyLock<usize> = LazyLock::new(|| {
        let text = lazysimd::scan::get_text();
        lazysimd::get_offset_neon(&text, "fd 7b be a9 f3 0b 00 f9 fd 03 00 91 f3 03 00 aa ?? ?? ?? ?? ?? ?? ?? ?? c0 00 80 52 ?? ?? ?? ?? e0 03 13 aa ?? ?? ?? ?? f3 0b 40 f9 00 00 00 12 fd 7b c2 a8 c0 03 5f d6").unwrap()
    });

    *OFFSETS
}
