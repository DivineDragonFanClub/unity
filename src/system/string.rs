use std::{fmt::{Display, Formatter}, str::FromStr};

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

impl Il2CppString {
    /// Create a new instance of a SystemString using the provided value.
    /// 
    /// Internally turned into a `CString`, so make sure the provided value is a valid UTF-8 string.
    /// 
    /// Example:
    ///
    /// ```
    /// let string = Il2CppString::new("A new string");
    /// ```j
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
