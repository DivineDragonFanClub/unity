use std::{fmt::{Display, Formatter}, str::FromStr};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

use crate::prelude::{Il2CppClass, Il2CppClassData, Il2CppObject, OptionalMethod};

/// A type alias for `Il2CppObject<SystemString>`.
/// 
/// Represents a C# string used by Il2Cpp.
pub type Il2CppString = Il2CppObject<SystemString>;

/// Represents a C# String used by Il2Cpp.
/// 
/// It is rarely needed to manipulate this directly.  
/// Prefer using [`Il2CppString`] instead.
#[repr(C)]
#[derive(Clone)]
pub struct SystemString {
    pub len: i32,
    pub string: [u16; 0],
}

impl Il2CppClassData for Il2CppString {
    const NAMESPACE: &'static str = "System";
    const CLASS: &'static str = "String";

    fn class() -> &'static Il2CppClass {
        static CLASS_TYPE: std::sync::LazyLock<&'static mut Il2CppClass> = std::sync::LazyLock::new(|| {
            Il2CppClass::from_name("System", "String")
                .expect(&format!("Failed to find class {}.{}", "System", "String"))
        });

        &CLASS_TYPE
    }

    fn class_mut() -> &'static mut Il2CppClass {
        Self::class().clone()
    }
}

#[crate::from_offset("System", "String", "Copy")]
fn system_string_copy(string: &Il2CppString, method_info: OptionalMethod) -> &'_ mut Il2CppString;

#[crate::from_offset("System", "String", "Clone")]
fn system_string_clone(this: &Il2CppString, method_info: OptionalMethod) -> &'_ mut Il2CppString;

// #[crate::from_offset("System", "String", "Replace")]
#[skyline::from_offset(0x3773720)]
fn system_string_replace_str(this: &mut Il2CppString, old_value: &Il2CppString, new_value: &Il2CppString, method_info: OptionalMethod) -> &'static mut Il2CppString;

#[crate::from_offset("System", "String", "Contains")]
fn system_string_contains(this: &Il2CppString, value: &Il2CppString, method_info: OptionalMethod) -> bool;

#[crate::from_offset("System", "String", "ToLower")]
fn system_string_to_lower(this: &Il2CppString, method_info: OptionalMethod) -> &'_ mut Il2CppString;

#[crate::from_offset("System", "String", "StartsWith")]
fn system_string_starts_with(this: &Il2CppString, value: &Il2CppString, method_info: OptionalMethod) -> bool;

#[crate::from_offset("System", "String", "Equals")]
fn system_string_equals(a: &Il2CppString, b: &Il2CppString, method_info: OptionalMethod) -> bool;

// This might use a This argument but Ghidra shows it as __this.
#[crate::from_offset("System", "String", "GetHashCode")]
fn system_string_get_hash_code(this: &Il2CppString, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x44a168)]
pub fn string_new_size(length: i32, method_info: OptionalMethod) -> Option<&'static mut Il2CppString>;

impl Il2CppString {
    /// Create a new instance of a SystemString using the provided value.
    /// 
    /// Internally turned into a `CString`, so make sure the provided value is a valid UTF-8 string.
    /// 
    /// Example:
    ///
    /// ```
    /// let string = Il2CppString::new("A new string");
    /// ```
    pub fn new<'a>(string: impl AsRef<str>) -> &'a Il2CppString {
        let cock = std::ffi::CString::new(string.as_ref()).unwrap();
        unsafe { string_new(cock.as_bytes_with_nul().as_ptr()) }
    }

    pub fn new_static(string: impl AsRef<str>) -> &'static mut Il2CppString {
        let cock = std::ffi::CString::new(string.as_ref()).unwrap();
        unsafe { string_new(cock.as_bytes_with_nul().as_ptr()) }
    }

    #[deprecated(note = "Use Il2CppString::to_string instead")]
    pub fn get_string(&self) -> Result<String, std::string::FromUtf16Error> {
        if self.len == 0 {
            Ok(String::new())
        } else {
            unsafe { String::from_utf16(std::slice::from_raw_parts(self.string.as_ptr(), self.len as _)) }
        }
    }

    pub fn to_string(&self) -> String {
        if self.len == 0 {
            String::new()
        } else {
            unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(self.string.as_ptr(), self.len as _)) }
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        if self.len == 0 {
            Vec::new()
        }  else {
            let utf16_buf = unsafe { std::slice::from_raw_parts(self.string.as_ptr(), self.len as _) };
            utf16_to_utf8(utf16_buf).unwrap()
        }
    }

    pub fn to_lowercase(&self) -> &'_ mut Il2CppString {
        unsafe { system_string_to_lower(self, None) }
    }

    pub fn starts_with<'a>(&self, value: impl Into<&'a Il2CppString>) -> bool {
        unsafe { system_string_starts_with(self, value.into(), None) }
    }

    pub fn contains<'a>(&self, value: impl Into<&'a Il2CppString>) -> bool {
        unsafe { system_string_contains(self, value.into(), None) }
    }

    pub fn replace<'a>(&mut self, old_value: impl Into<&'a Il2CppString>, new_value: impl Into<&'a Il2CppString>) -> &'_ mut Il2CppString {
        unsafe { system_string_replace_str(self, old_value.into(), new_value.into(), None) }
    }

    /// Provides a new instance of the Il2CppString, separate from the original.
    pub fn clone(&self) -> &'_ Il2CppString {
        // Yes.
        unsafe { system_string_copy(self, None) }
    }

    pub fn clone_mut(&mut self) -> &'_ mut Il2CppString {
        // Yes.
        unsafe { system_string_copy(self, None) }
    }

    pub fn copy(&self) -> &'_ Il2CppString {
        // Yes.
        unsafe { system_string_clone(self, None) }
    }

    pub fn copy_mut(&mut self) -> &'_ mut Il2CppString {
        // Yes.
        unsafe { system_string_clone(self, None) }
    }

    pub fn get_hash_code(&self) -> i32 {
        unsafe { system_string_get_hash_code(self, None) }
    }
}

impl Display for Il2CppString {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<T: AsRef<str>> From<T> for &'_ Il2CppString {
    fn from(value: T) -> Self {
        Il2CppString::new(value)
    }
}

impl<T: AsRef<str>> From<T> for &'_ mut Il2CppString {
    fn from(value: T) -> Self {
        Il2CppString::new_static(value)
    }
}

impl PartialEq for Il2CppString {
    fn eq(&self, other: &Self) -> bool {
        unsafe { system_string_equals(self, other, None) }
    }
}

impl FromStr for &'_ Il2CppString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Il2CppString::new(s))
    }
}

#[lazysimd::from_pattern("ff 03 01 d1 fd 7b 02 a9 fd 83 00 91 f4 4f 03 a9 f3 03 00 aa ?? ?? ?? ?? 01 7c 40 92 e8 23 00 91 e0 03 13 aa f4 23 00 91 ?? ?? ?? ?? e8 23 40 39 0b fd 41 d3 e9 0f 40 f9")]
fn string_new<'a>(c_str: *const u8) -> &'a mut Il2CppString;

fn utf16_to_utf8(input: &[u16]) -> Option<Vec<u8>> {
    // Check if the string only contains ASCII and accelerate the conversion if so
    if input.iter().all(|&c| c <= 0x7F) {
        utf16_to_utf8_ascii_neon(input)
    } else {
        Some(String::from_utf16(input).ok()?.into_bytes())
    }
}

fn utf16_to_utf8_ascii_neon(input: &[u16]) -> Option<Vec<u8>> {
    let len = input.iter().position(|&c| c == 0).unwrap_or(input.len());
    let input = &input[..len];

    let mut out = Vec::with_capacity(len);

    unsafe {
        out.set_len(len);

        let mut i = 0;
        let mut dst: *mut u8 = out.as_mut_ptr();

        while i + 8 <= len {
            let ptr = input.as_ptr().add(i);

            let chunk: uint16x8_t = vld1q_u16(ptr);

            let mask = vcgtq_u16(chunk, vdupq_n_u16(0x7F));
            if vmaxvq_u16(mask) != 0 {
                return None;
            }
            
            let narrowed: uint8x8_t = vmovn_u16(chunk);
            
            vst1_u8(dst.add(i), narrowed);

            i += 8;
        }
        
        for j in i..len {
            let c = *input.get_unchecked(j);
            if c > 0x7F {
                return None;
            }
            *dst.add(j) = c as u8;
        }
    }

    Some(out)
}

pub fn u8_to_u16_neon(input: &[u8]) -> Vec<u16> {
    let len = input.len();
    let mut out = Vec::with_capacity(len);

    unsafe {
        out.set_len(len);

        let mut i = 0;

        let src = input.as_ptr();
        let dst: *mut u16 = out.as_mut_ptr();
        
        while i + 16 <= len {
            let p = src.add(i);
                
            let a = vld1_u8(p);
            let b = vld1_u8(p.add(8));
                
            let a16 = vmovl_u8(a);
            let b16 = vmovl_u8(b);
                
            vst1q_u16(dst.add(i), a16);
            vst1q_u16(dst.add(i + 8), b16);
                
            i += 16;
        }
        
        while i < len {
            *dst.add(i) = *src.add(i) as u16;
            i += 1;
        }
    }

    out
}
