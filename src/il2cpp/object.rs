use std::ops::{Deref, DerefMut};

use crate::{Il2CppResult, Il2CppError};

use super::{api, class::{Il2CppClass, Il2CppClassData}};

/// A type alias for `Il2CppObject<Array<T>>`.
pub type Il2CppArray<T> = Array<T>;

/// Wrapper structure for a class instance provided by Il2Cpp.
/// 
/// It contains a pointer to the class it represents as well as its own copy of the fields in the class.  
/// Every class instance is represented by this type and passed by reference.
/// 
/// Define the fields of the class as a structure and pass them as a generic.
/// 
/// Example:
///
/// ```
/// pub fn hooked_method(proc: &Il2CppObject<ProcInst>) {
/// // ...
/// }
/// ```
#[repr(C)]
pub struct Il2CppObject<T> {
    /// The class this instance refers to.
    /// Be aware that editing it means editing every other object using the same class.  
    /// Use carefully.
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: T,
}

impl<T> Deref for Il2CppObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl<T> DerefMut for Il2CppObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}

impl<T> AsRef<T> for Il2CppObject<T> {
    fn as_ref(&self) -> &T {
        &self.fields
    }
}

impl<T> AsMut<T> for Il2CppObject<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.fields
    }
}

impl<T: Sized> Il2CppObjectMethods for Il2CppObject<T> {
    fn get_class(&self) -> &Il2CppClass {
        self.klass
    }
    fn get_class_mut(&mut self) -> &mut Il2CppClass {
        self.klass
    }
}

impl<T> Il2CppObject<T> {
    pub fn from_class(class: &Il2CppClass) -> Il2CppResult<&'static mut T> {
        unsafe { api::object_new(class) }.ok_or(Il2CppError::FailedInstantiation(class.get_name()))
    }
}

pub trait Il2CppObjectMethods {
    fn get_class(&self) -> &Il2CppClass;
    fn get_class_mut(&mut self) -> &mut Il2CppClass;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Il2CppArrayBounds {
    length: usize,
    lower_bounds: u32,
}

#[crate::class("System", "Array")]
pub struct Array<T> {
    bounds: &'static Il2CppArrayBounds,
    pub max_length: usize,
    pub m_items: [T; 0],
}

impl<T> Deref for ArrayFields<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.m_items.as_ptr(), self.max_length) }
    }
}

impl<T> DerefMut for ArrayFields<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.m_items.as_mut_ptr(), self.max_length) }
    }
}

/// Trait to abstract away the new methods for value types vs reference ones.
pub trait ArrayInstantiator<T> {
    /// Create an empty Il2CppArray capable of holding the provided amount of entries.
    /// 
    /// Arguments:
    ///
    /// * `class`: The class of the elements that are going to be stored.
    /// * `capacity`: The maximum amount of element that can be stored.
    /// 
    /// Example:
    /// 
    /// ```
    /// let new_array: Il2CppArray<u8> = Il2CppArray::<u8>::new(SystemByte::get_class(), 69).unwrap();
    /// ```
    fn new(capacity: usize) -> Il2CppResult<&'static mut Self>;

    /// Create a new Il2CppArray by copying the content of a slice into it.
    /// 
    /// Arguments:
    ///
    /// * `class`: The class of the elements that are going to be stored.
    /// * `slice`: The slice that'll be copied into the Il2CppArray.
    /// 
    /// Example:
    /// 
    /// ```
    /// let mut slice: &mut [u8] = &[0x1, 0x2, 0x3];
    /// let new_array: Il2CppArray<u8> = Il2CppArray::<u8>::new_from(SystemByte::get_class(), slice).unwrap();
    /// ```
    /// 
    /// Note that this method takes ownership of the slice, so you won't be able to use it afterwards.
    fn from_slice(slice: impl AsMut<[T]>) -> Il2CppResult<&'static mut Self>;
}

impl ArrayInstantiator<u8> for Array<u8> {
    /// Create an empty Il2CppArray capable of holding the provided amount of entries.
    /// 
    /// Arguments:
    ///
    /// * `class`: The class of the elements that are going to be stored.
    /// * `capacity`: The maximum amount of element that can be stored.
    /// 
    /// Example:
    /// 
    /// ```
    /// let new_array: Il2CppArray<u8> = Il2CppArray::<u8>::new(SystemByte::get_class(), 69).unwrap();
    /// ```
    fn new(capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new(u8::class(), capacity)
    }

    /// Create a new Il2CppArray by copying the content of a slice into it.
    /// 
    /// Arguments:
    ///
    /// * `class`: The class of the elements that are going to be stored.
    /// * `slice`: The slice that'll be copied into the Il2CppArray.
    /// 
    /// Example:
    /// 
    /// ```
    /// let mut slice: &mut [u8] = &[0x1, 0x2, 0x3];
    /// let new_array: Il2CppArray<u8> = Il2CppArray::<u8>::new_from(SystemByte::get_class(), slice).unwrap();
    /// ```
    /// 
    /// Note that this method takes ownership of the slice, so you won't be able to use it afterwards.
    fn from_slice(mut slice: impl AsMut<[u8]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new(u8::class(), slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }
}

impl<T: Il2CppClassData> ArrayInstantiator<&'static mut T> for Array<&'static mut T> {
    /// Create an empty Il2CppArray capable of holding the provided amount of entries.
    /// 
    /// Arguments:
    ///
    /// * `class`: The class of the elements that are going to be stored.
    /// * `capacity`: The maximum amount of element that can be stored.
    /// 
    /// Example:
    /// 
    /// ```
    /// let new_array: Il2CppArray<u8> = Il2CppArray::<u8>::new(SystemByte::get_class(), 69).unwrap();
    /// ```
    fn new(capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new(T::class(), capacity)
    }

    /// Create a new Il2CppArray by copying the content of a slice into it.
    /// 
    /// Arguments:
    ///
    /// * `class`: The class of the elements that are going to be stored.
    /// * `slice`: The slice that'll be copied into the Il2CppArray.
    /// 
    /// Example:
    /// 
    /// ```
    /// let mut slice: &mut [u8] = &[0x1, 0x2, 0x3];
    /// let new_array: Il2CppArray<u8> = Il2CppArray::<u8>::new_from(SystemByte::get_class(), slice).unwrap();
    /// ```
    /// 
    /// Note that this method takes ownership of the slice, so you won't be able to use it afterwards.
    fn from_slice(mut slice: impl AsMut<[&'static mut T]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new(T::class(), slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }
}

impl<T> Array<T> {
    pub fn new_specific(class: &Il2CppClass, capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new_specific(class, capacity)
    }

    pub fn new_from_token(token: u64, capacity: usize) -> Il2CppResult<&'static mut Self> {
        let temp = token;
        let fake_class = unsafe { &*(temp as *mut Il2CppClass) };
        array_new_specific(fake_class, capacity)
    }

    /// Takes a mutable slice and allocates a new Il2CppArray filled with its content.
    ///
    /// This is partially needed because we do not implement Clone on Il2CppObject.
    pub fn new_specific_from(class: &Il2CppClass, mut slice: impl AsMut<[T]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new_specific(class, slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }

    /// Create a new Vec filled with the content from the Il2CppArray.
    ///
    /// This is partially needed because we do not implement Clone on Il2CppObject.
    pub fn to_vec(&mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        unsafe { vec.set_len(self.len()) }
        // TODO: Try to improve this with a memcpy, since we have the same size
        self.swap_with_slice(vec.as_mut());
        vec
    }
}

fn array_new_specific<T>(array_typeinfo: &Il2CppClass, length: usize) -> Il2CppResult<&'static mut Il2CppArray<T>> {
    unsafe { api::array_new_specific(array_typeinfo, length) }.ok_or(Il2CppError::FailedArrayInstantiation)
}

fn array_new<T>(element_typeinfo: &Il2CppClass, length: usize) -> Il2CppResult<&'static mut Il2CppArray<T>> {
    unsafe { api::array_new(element_typeinfo, length) }.ok_or(Il2CppError::FailedArrayInstantiation)
}
