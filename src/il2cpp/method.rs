use std::ffi::CStr;

use super::{class::Il2CppClass, Il2CppType};

/// A type alias for `Option<&MethodInfo>`. Useful when hooking Il2Cpp methods.
pub type OptionalMethod = Option<&'static MethodInfo>;

/// Type representing the reflection information of a C# method.
/// 
/// Can be used to query various things such as the name, argument count and much more.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MethodInfo {
    pub method_ptr: *mut u8,
    pub invoker_method: *const u8,
    pub name: *const u8,
    pub class: Option<&'static Il2CppClass>,
    pub return_type: *const u8,
    pub parameters: *const ParameterInfo,
    pub info_or_definition: *const u8,
    pub generic_method_or_container: *const u8,
    pub token: u32,
    pub flags: u16,
    pub iflags: u16,
    pub slot: u16,
    pub parameters_count: u8,
    pub bitflags: u8,
}

unsafe impl Send for MethodInfo {}
unsafe impl Sync for MethodInfo {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ParameterInfo {
    pub name: *const u8,
    pub position: i32,
    pub token: u32,
    pub parameter_type: &'static Il2CppType,
}

impl MethodInfo {
    pub fn new() -> Self {
        Self {
            method_ptr: 0 as _,
            invoker_method: 0 as _,
            name: 0 as _,
            class: None,
            return_type: 0 as _,
            parameters: 0 as _,
            info_or_definition: 0 as _,
            generic_method_or_container: 0 as _,
            bitflags: 0,
            flags: 0,
            iflags: 0,
            parameters_count: 0,
            slot: 0,
            token: 0,
        }
    }

    pub fn new_from(base: Self) -> Self {
        Self {
            invoker_method: 0 as _,
            bitflags: 0,
            flags: 0,
            iflags: 0,
            slot: 0,
            token: 0,
            ..base
        }
    }
}

impl MethodInfo {
    /// Get the name of the method, if set.
    pub fn get_name(&self) -> Option<String> {
        if self.name.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8_lossy(CStr::from_ptr(self.name as _).to_bytes()).to_string() })
        }
    }

    /// Get the parameters expected by the method.
    pub fn get_parameters(&self) -> &[ParameterInfo] {
        unsafe { std::slice::from_raw_parts(self.parameters, self.parameters_count as _) }
    }
}

impl ParameterInfo {
    /// Get the name of the parameter, if set.
    pub fn get_name(&self) -> Option<String> {
        if self.name.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8_lossy(CStr::from_ptr(self.name as _).to_bytes()).to_string() })
        }
    }
}

