#![allow(dead_code)]

use std::sync::LazyLock;

use lazysimd;

use super::*;
use crate::cppvector::CppVector;

#[lazysimd::from_pattern("fd 7b be a9 f3 0b 00 f9 fd 03 00 91 f3 03 00 aa ?? ?? ?? ?? ?? ?? ?? ?? c0 00 80 52 ?? ?? ?? ?? e0 03 13 aa ?? ?? ?? ?? f3 0b 40 f9 00 00 00 12 fd 7b c2 a8 c0 03 5f d6")]
pub(crate) fn init(domain_name: *const i8) -> i32;

// Get Image By Assembly Name*
#[lazysimd::from_pattern("ff 03 01 d1 fd 7b 01 a9 fd 43 00 91 f6 57 02 a9 f4 4f 03 a9 f3 03 00 aa ?? ?? ?? ?? ?? ?? ?? ?? 08 21 32 91 f4 03 00 aa ?? ?? ?? ?? df 02 08 eb 80 01 00 54 ?? ?? ?? ??")]
pub(crate) fn get_image_by_assembly_name(c_str: *const u8) -> &'static Il2CppImage;

#[lazysimd::from_pattern(
    "ff c3 02 d1 fd 7b 05 a9 fd 43 01 91 fc 6f 06 a9 fa 67 07 a9 f8 5f 08 a9 f6 57 09 a9 f4 4f 0a a9 f8 03 00 aa 16 0f 43 f8 f9 03 02 aa f4 03 01 aa"
)]
pub fn class_from_name(image: &Il2CppImage, namespace: *const u8, name: *const u8) -> Option<&'static mut Il2CppClass>;

#[lazysimd::from_pattern("ff 43 01 d1 fd 7b 01 a9 fd 43 00 91 f7 13 00 f9 f6 57 03 a9 f4 4f 04 a9 08 c8 44 39 f3 03 00 aa e8 02 10 37")]
pub(crate) fn object_new<T>(klass: &Il2CppClass) -> Option<&'static mut T>;

#[lazysimd::from_pattern(
    "ff 43 01 d1 fd 7b 01 a9 fd 43 00 91 f8 5f 02 a9 f6 57 03 a9 f4 4f 04 a9 08 c8 44 39 f3 03 03 2a f6 03 02 2a f4 03 01 aa f5 03 00 aa"
)]
pub(crate) fn get_method_from_name_flags(
    class: &Il2CppClass,
    method_name: *const u8,
    args_count: usize,
    flags: u32,
) -> Option<&'static mut MethodInfo>;

#[skyline::from_offset(0x42911c)]
pub(crate) fn assembly_getallassemblies() -> &'static CppVector<&'static Il2CppAssembly>;

#[lazysimd::from_pattern("ff 03 01 d1 fd 7b 01 a9 fd 43 00 91 f6 57 02 a9 f4 4f 03 a9 08 c8 44 39 f4 03 01 aa f3 03 00 aa")]
pub(crate) fn array_new_specific<T>(array_typeinfo: &Il2CppClass, length: usize) -> Option<&'static mut Il2CppArray<T>>;

#[lazysimd::from_pattern(
    "fd 7b be a9 f3 0b 00 f9 fd 03 00 91 f3 03 01 aa 21 00 80 52 e2 03 1f 2a ?? ?? ?? ?? e1 03 13 aa f3 0b 40 f9 fd 7b c2 a8"
)]
pub(crate) fn array_new<T>(element_typeinfo: &Il2CppClass, length: usize) -> Option<&'static mut Il2CppArray<T>>;

#[lazysimd::from_pattern(
    "ff 03 01 d1 fd 7b 01 a9 fd 43 00 91 f5 13 00 f9 f4 4f 03 a9 ?? ?? ?? ?? ?? ?? ?? ?? a0 0f 00 f9 e0 03 13 aa ff 07 00 f9"
)]
pub(crate) fn type_get_object(ty: &Il2CppType) -> Option<&'static mut Il2CppReflectionType>;

fn domain_getcurrent_scan() -> usize {
    static OFFSETS: LazyLock<usize> = LazyLock::new(|| {
        let text = lazysimd::scan::get_text();
        lazysimd::get_offset_neon(
            &text,
            "fd 7b be a9 f3 0b 00 f9 fd 03 00 91 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 80 00 00 b4 f3 0b 40 f9 fd 7b c2 a8 c0 03 5f d6",
        )
        .unwrap()
    });

    *OFFSETS
}

#[lazysimd::from_pattern(
    "ff 03 01 d1 fd 7b 01 a9 fd 43 00 91 f6 57 02 a9 f4 4f 03 a9 f3 03 00 aa e0 03 1f aa 68 2a 40 39 08 05 00 51 1f 75 00 71"
)]
pub(crate) fn class_from_il2cpptype(ty: &Il2CppType) -> Option<&'static mut Il2CppClass>;

#[lazysimd::from_pattern(
    "fd 7b bd a9 f5 0b 00 f9 fd 03 00 91 f4 4f 02 a9 08 c8 44 39 08 03 10 37 ?? ?? ?? ?? ?? ?? ?? ?? f3 03 00 aa b5 0f 00 f9"
)]
pub(crate) fn class_init(class: &Il2CppClass);

#[lazysimd::from_pattern(
    "fd 7b bc a9 f7 0b 00 f9 fd 03 00 91 f6 57 02 a9 f4 4f 03 a9 08 08 40 b9 f4 03 02 aa f3 03 01 aa e8 0b f0 37 f5 03 03 2a f6 03 00 aa"
)]
pub(crate) fn field_set_value_raw(ty: &Il2CppType, dest: *const u8, value: *const u8, deref_pointer: bool);
