use crate::prelude::{Il2CppArray, Il2CppClassData, MethodInfo};
use std::{marker::PhantomData, ops::{Deref, DerefMut}};

pub mod string;
pub use string::Il2CppString;

#[repr(C)]
#[crate::class("System", "Type")]
pub struct SystemType { }

#[repr(C)]
#[crate::class("System", "Byte")]
pub struct SystemByte { }


#[crate::from_offset("System", "RuntimeType", "MakeGenericType")]
pub fn runtime_type_make_generic_type(gt: *const u8, ty: *const u8);

#[repr(C)]
#[crate::class("System.Collections.Generic", "List`1")]
pub struct SystemList {}

/// The Il2Cpp equivalent of a C# List, similar to a Rust Vec.
/// 
/// Internally backed by a [`Il2CppArray`](crate::il2cpp::object::Il2CppArray), this class keeps track of how many entries are in the array.  
/// This means you do not want to directly edit the array unless you also increase the size field.
#[repr(C)]
#[crate::class("System.Collections.Generic", "List`1")]
pub struct List<T: 'static> {
    pub items: &'static mut Il2CppArray<&'static mut T>,
    pub size: u32,
    version: u32,
    sync_root: *const u8,
}

impl<T: 'static> Deref for ListFields<T> {
    type Target = [&'static mut T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.items.m_items.as_ptr(), self.size as usize) }
    }
}

impl<T: 'static> DerefMut for ListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.items.m_items.as_mut_ptr(), self.size as usize) }

    }
}

impl<T: crate::il2cpp::class::Il2CppClassData> List<T> {
    pub fn with_capacity(capacity: i32) -> Result<&'static mut Self, Il2CppError> {
        let list_class = crate::il2cpp::class::make_generic(&SystemList::class(), &[T::class()])?;
        //let list_class = crate::get_generic_class!(SystemList<T>).unwrap();
        let list = crate::il2cpp::instantiate_class::<Self>(&list_class)?;

        // Get constructor that expects a capacity
        let method = list.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from(".ctor")) && method.parameters_count == 1)
            .unwrap();

        let ctor = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, i32, &MethodInfo)>(
                method.method_ptr,
            )
        };

        ctor(list, capacity, method);

        list
    }
    
    pub fn resize(&mut self, length: usize) {
        if self.items.len() != length {
            let new_array = crate::il2cpp::object::Il2CppArray::new_specific(self.items.get_class(), length as _).unwrap();
            new_array[..self.items.len()].swap_with_slice(self.items);
            self.items = new_array;
        }
    }

    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Add")))
            .unwrap();
        
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_ptr,
            )
        };

        add(self, element, method);
    }

    pub fn insert(&mut self, index: i32, element: &'static mut T) {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Insert")))
            .unwrap();
        
        let insert = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, i32, &'static mut T, &MethodInfo)>(
                method.method_ptr,
            )
        };

        insert(self, index, element, method);
    }
    pub fn len(&self) -> usize {
        self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }

    pub fn clear(&mut self) {
        self.get_class().get_virtual_method("Clear").map(|method| {
            let clear = unsafe { std::mem::transmute::<_, extern "C" fn(&List<T>, &MethodInfo)>(method.method_info.method_ptr) };
            clear(&self, method.method_info);
        }).unwrap();
    }
}

pub trait ListVirtual<T>: Il2CppClassData {
    fn add(&mut self, element: &'static mut T) {
        let method = Self::class().get_virtual_method("Add").unwrap();
        
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, element, method.method_info);
    }
}

#[repr(C)]
#[crate::class("System.Collections.Generic", "Stack`1")]
pub struct Stack<T: 'static> {
    pub items: &'static mut Il2CppArray<&'static mut T>,
    pub size: u32,
    version: u32,
    sync_root: *const u8,
}

impl<T: 'static> Deref for StackFields<T> {
    type Target = [&'static mut T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.items.m_items.as_ptr(), self.size as usize) }
    }
}

impl<T: 'static> DerefMut for StackFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.items.m_items.as_mut_ptr(), self.size as usize) }

    }
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<&'static mut T> {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Pop")))
            .unwrap();
        
        let pop = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &MethodInfo) -> Option<&'static mut T>>(
                method.method_ptr,
            )
        };

        pop(self, method)
    }

    pub fn len(&self) -> usize {
      self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }
}

#[crate::class("System.Collections.Generic", "Dictionary`1")]
pub struct Dictionary<TKey, TValue> {
    lol: PhantomData<(TKey, TValue)>
}

impl<TKey, TValue> Dictionary<TKey, TValue> {
    pub fn add(&self, key: TKey, value: TValue) {
        let method = self.get_class()
            .get_virtual_method("Add")
            .unwrap();
        
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, TKey, TValue, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, key, value, method.method_info);
    }
    pub fn remove(&self, key: TKey) {
        let method = self.get_class()
            .get_virtual_method("Remove")
            .unwrap();
    
        let remove = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, TKey, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };
        remove(self, key, method.method_info);
    }
    pub fn get_count(&self) -> i32 {
        let method = self.get_class()
            .get_virtual_method("get_Count")
            .unwrap();
    
        let count = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self) -> i32 >(
                method.method_info.method_ptr,
            )
        };
        count(self)
    }
    pub fn try_get_value<'a>(&self, key: TKey, value: &'a mut TValue) -> bool {
        let method = self.get_class()
            .get_virtual_method("TryGetValue")
            .unwrap();
        
        let try_get_value = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, TKey, &mut TValue, &MethodInfo) -> bool>(
                method.method_info.method_ptr,
            )
        };

        try_get_value(self, key, value, method.method_info)
    }
}
