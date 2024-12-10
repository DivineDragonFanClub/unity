use std::ffi::CStr;
use std::fmt::{Debug, Formatter, Write};
use super::{
    api,
    assembly::Il2CppImage,
    method::MethodInfo,
    object::Il2CppArray,
    Il2CppType,
};
use crate::{Il2CppResult, Il2CppError, system::{SystemType, runtime_type_make_generic_type}};

#[repr(C)]
pub struct Il2CppClass1 {
    pub image: &'static Il2CppImage,
    pub gc_desc: *const u8,
    name: *const u8,
    namespace: *const u8,
    pub byval_arg: Il2CppType,
    this_arg: Il2CppType,
    pub element_class: &'static Il2CppClass,
    _1_start: [u8; 0x10],
    pub parent: &'static Il2CppClass,
    pub generic_class: Option<&'static Il2CppGenericClass>,
    _1_end: [u8; 0x18],
    pub fields: *const FieldInfo,
    pub events: *const u8,
    pub properties: *const u8,
    pub methods: *const &'static MethodInfo,
    pub nested_types: *const &'static Il2CppClass,
    implemented_interfaces: *const u8,
    interface_offsets: *const u8,
}

#[repr(C)]
pub struct Il2CppClass2 {
    _2_start: [u8; 0x30],
    pub instance_size: u32,
    pub actual_size: u32,
    __: [u8; 0x18],
    pub token: u32,
    pub method_count: u16,
    property_count: u16,
    field_count: u16,
    event_count: u16,
    pub nested_type_count: u16,
    pub vtable_count: u16,
    interfaces_count: u16,
    interface_offsets_count: u16,
    type_hierarchy_depth: u8,
    generic_recursion_depth: u8,
    pub rank: u8,
    _2_end: [u8; 0x9],
}

#[repr(C)]
pub struct Il2CppClass {
    pub _1: Il2CppClass1,
    pub static_fields: *mut (),
    pub rgctx_data: &'static mut Il2CppRGCTXData,
    pub _2: Il2CppClass2,
    vtable: [VirtualInvoke; 0],
}

unsafe impl Send for Il2CppClass {}
unsafe impl Sync for Il2CppClass {}

#[repr(C)]
pub struct FieldInfo {
    name: *const u8,
    ty: &'static Il2CppType,
    parent: &'static Il2CppClass,
    offset: i32,
    token: u32,
}

impl Debug for FieldInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}, //{}", self.get_name().unwrap_or_default(), unsafe { self.ty.data.class_index }, self.offset))
    }
}

impl FieldInfo {

    pub fn is_instance(&self) -> bool {
        // TODO: Actually implement a bitfield
        (self.ty.bits >> 16) & 0x0010 == 0
    }

    pub fn get_name(&self) -> Option<String> {
        if self.name.is_null() {
            None
        } else {
            Some(unsafe { String::from_utf8_lossy(CStr::from_ptr(self.name as _).to_bytes()).to_string() })
        }
    }
}

#[repr(C)]
pub struct Il2CppRGCTXData {
    // dummy: *const u8,
    // pub method: &'static MethodInfo,
    // ty: *const u8,
    pub class_self: &'static mut Il2CppClass,
    pub class_t: &'static mut Il2CppClass,
    osef: [&'static mut MethodInfo; 2],
    pub get_instance: &'static mut MethodInfo,
    // ...
}

#[repr(C)]
pub struct Il2CppGenericClass {
    type_definition_idx: i32,
    class_inst: *const u8,
    method_inst: *const u8,
    pub cached_class: *const Il2CppClass,
}

#[skyline::from_offset(0x18eeb30)]
fn memcpy<T>(dest: &mut T, src: &T, size: usize) -> &'static mut T;

#[skyline::from_offset(0x474370)]
fn gc_malloc_kind<T>(size: usize, kind: u32) -> &'static mut T;

impl Il2CppClass {
    pub fn from_name(namespace: impl AsRef<str>, name: impl AsRef<str>) -> Il2CppResult<&'static mut Self> {
        get_class_from_name(namespace, name)
    }

    pub fn from_il2cpptype(ty: &Il2CppType) -> Il2CppResult<&'static mut Self> {
        class_from_il2cpptype(ty)
    }

    pub fn from_system_type(ty: &Il2CppReflectionType) -> Il2CppResult<&'static mut Self> {
        class_from_system_type(ty)
    }

    pub fn with_generic_type<'a>(&self, args: impl AsRef<[&'a Il2CppClass]>) -> Il2CppResult<&'static mut Il2CppClass> {
        make_generic(self, args)
    }

    pub fn clone(&self) -> &'static mut Il2CppClass {
        let size = 0x138 + (0x10 * self._2.vtable_count) as usize;
        let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();

        unsafe {
            // Malloc kind is "Normal" here, meaning the class and its inner pointers can be managed and freed by the Garbage Collector (BoehmGC)
            let dest = &mut *(std::alloc::alloc(layout) as *mut Il2CppClass);
            memcpy(dest, self, size);
            dest
        }
    }

    // TODO: Should return a Result instead but this needs further testing
    // pub fn get_generic_typeinfo(&self) -> Option<&'static Il2CppClass> {
    //     if self._1.generic_class.is_some() {
    //         let class = self.rgctx_data as *const [&'static Il2CppClass; 2];
    //         Some(unsafe { (*class)[1] })
    //     } else {
    //         None
    //     }
    // }

    pub fn get_static_fields<T>(&self) -> &T {
        unsafe { std::mem::transmute(self.static_fields) }
    }

    pub fn get_static_fields_mut<T>(&self) -> &mut T {
        unsafe { std::mem::transmute(self.static_fields) }
    }

    pub fn get_type(&self) -> &Il2CppType {
        &self._1.byval_arg
    }

    pub fn get_name(&self) -> String {
        unsafe { String::from_utf8_lossy(CStr::from_ptr(self._1.name as _).to_bytes()).to_string() }
    }

    pub fn get_namespace(&self) -> String {
        unsafe { String::from_utf8_lossy(CStr::from_ptr(self._1.namespace as _).to_bytes()).to_string() }
    }

    pub fn get_vtable(&self) -> &[VirtualInvoke] {
        unsafe { std::slice::from_raw_parts(self.vtable.as_ptr(), self._2.vtable_count as _) }
    }

    pub fn get_vtable_mut(&mut self) -> &mut [VirtualInvoke] {
        unsafe { std::slice::from_raw_parts_mut(self.vtable.as_mut_ptr(), self._2.vtable_count as _) }
    }

    pub fn get_virtual_method(&self, name: impl AsRef<str>) -> Option<&VirtualInvoke> {
        self.get_vtable()
            .iter()
            .find(|method| method.get_name().unwrap_or_default() == name.as_ref())
    }

    pub fn get_virtual_method_mut(&mut self, name: impl AsRef<str>) -> Option<&mut VirtualInvoke> {
        self.get_vtable_mut()
            .iter_mut()
            .find(|method| method.get_name().unwrap_or_default() == name.as_ref())
    }

    /// Returns a slice containing the fields of the class, static and constants included
    pub fn get_fields(&self) -> &[FieldInfo] {
        unsafe { std::slice::from_raw_parts(self._1.fields, self._2.field_count as _) }
    }

    /// Returns a Iterator over the fields that are not static or constants
    pub fn get_instance_fields(&self) -> impl Iterator<Item = &FieldInfo> {
        self.get_fields().iter().filter(|field| field.is_instance() && field.offset != 0)
    }

    pub fn get_methods(&self) -> &[&'static MethodInfo] {
        unsafe { std::slice::from_raw_parts(self._1.methods, self._2.method_count as _) }
    }

    pub fn get_nested_types(&self) -> &[&'static Il2CppClass] {
        unsafe { std::slice::from_raw_parts(self._1.nested_types, self._2.nested_type_count as _) }
    }

    pub fn get_method_from_name(&self, name: impl AsRef<str>, args_count: usize) -> Il2CppResult<&'static mut MethodInfo> {
        self.get_method_from_name_with_flag(name, args_count, 0)
    }

    pub fn get_method_from_name_with_flag(&self, name: impl AsRef<str>, args_count: usize, flag: u32) -> Il2CppResult<&'static mut MethodInfo> {
        let name = std::ffi::CString::new(name.as_ref()).unwrap();

        unsafe { api::get_method_from_name_flags(self, name.as_ptr() as _, args_count, flag) }.ok_or(Il2CppError::MissingMethod)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VirtualInvoke {
    pub method_ptr: *mut u8,
    pub method_info: &'static MethodInfo,
}

impl VirtualInvoke {
    pub fn get_name(&self) -> Option<String> {
        self.method_info.get_name()
    }
}

#[repr(C)]
#[crate::class("System", "ReflectionType")]
pub struct Il2CppReflectionType {
    ty: &'static Il2CppType,
}

fn get_class_from_name(namespace: impl AsRef<str>, name: impl AsRef<str>) -> Il2CppResult<&'static mut Il2CppClass> {
    // Search assemblies in reverse order as we're much more likely to want something from the game than Unity, speeding up our search.
    super::assembly::get_assemblies().iter().rev()
        .find_map(|assembly| {
            let namespace = std::ffi::CString::new(namespace.as_ref()).unwrap();
            let name = std::ffi::CString::new(name.as_ref()).unwrap();
            unsafe { api::class_from_name(assembly.image, namespace.as_ptr() as _, name.as_ptr() as _) }
        })
        .ok_or(Il2CppError::MissingClass(name.as_ref().to_string()))
}

fn get_class_method_from_name(
    namespace: impl AsRef<str>,
    name: impl AsRef<str>,
    method: impl AsRef<str>,
    args_count: usize,
) -> Il2CppResult<&'static mut MethodInfo> {
    get_class_from_name(namespace, name)
        .map(|class| class.get_method_from_name(method, args_count))?
}

fn class_from_il2cpptype(ty: &Il2CppType) -> Il2CppResult<&'static mut Il2CppClass> {
    let class = unsafe { api::class_from_il2cpptype(ty) }
        .ok_or(Il2CppError::MissingClassForType)?;

    unsafe {
        api::class_init(class);
    }

    Ok(class)
}

fn class_from_system_type(ty: &Il2CppReflectionType) -> Il2CppResult<&'static mut Il2CppClass> {
    class_from_il2cpptype(ty.ty)
}

struct MakeGenericTypeArgs<'a> {
    generic: &'a Il2CppReflectionType,
    args: &'a Il2CppArray<&'a mut Il2CppReflectionType>,
}

/// Helper method to call System.ReflectionType.MakeGenericType
pub fn make_generic_type(
    generic: &Il2CppReflectionType,
    args: &Il2CppArray<&mut Il2CppReflectionType>,
) -> Il2CppResult<&'static mut Il2CppReflectionType> {
    let make_generic_method = runtime_type_make_generic_type::get_ref();

    let params = MakeGenericTypeArgs {
        generic,
        args,
    };
    
    let runtime_invoke = unsafe {
        std::mem::transmute::<_, extern "C" fn(*const u8, &MethodInfo, Option<&()>, *const MakeGenericTypeArgs) -> Option<&'static mut Il2CppReflectionType>>(
            make_generic_method.invoker_method,
        )
    };

    runtime_invoke(make_generic_method.method_ptr, make_generic_method, None, &params).ok_or(Il2CppError::FailedMethodInvocation)
}

pub fn make_generic<'a>(generic_class: &Il2CppClass, types: impl AsRef<[&'a Il2CppClass]>) -> Il2CppResult<&'static mut Il2CppClass> {
    let types = types.as_ref();

    // Represent it as ReflectionType instead, as they have the same layout
    let array: &mut Il2CppArray<&mut Il2CppReflectionType> = Il2CppArray::new_specific(SystemType::class(), types.len())?;

    // Populate the array with the type of every argument
    for (arg, entry) in types.iter().zip(array.iter_mut()) {
        *entry = Il2CppType::get_object(arg.get_type())?;
    }

    let class_type = Il2CppType::get_object(generic_class.get_type())?;

    let reflection_type = make_generic_type(class_type, array).unwrap();

    Il2CppClass::from_system_type(reflection_type)
}

pub trait Il2CppClassData {
    const NAMESPACE: &'static str;
    const CLASS: &'static str;

    fn class() -> &'static Il2CppClass;

    fn class_mut() -> &'static mut Il2CppClass;

    fn instantiate() -> Il2CppResult<&'static mut Self>
    where
        Self: Sized,
    {
        super::instantiate_class(Self::class())
    }

    fn instantiate_as<T: 'static>() -> Il2CppResult<&'static mut T> {
        super::instantiate_class(Self::class())
    }
}

impl Il2CppClassData for u8 {
    const NAMESPACE: &'static str = "System";
    const CLASS: &'static str = "Byte";

    fn class() -> &'static Il2CppClass {
        static CLASS_TYPE: std::sync::LazyLock<&'static mut Il2CppClass> = std::sync::LazyLock::new(|| {
            Il2CppClass::from_name("System", "Byte")
                .expect(&format!("Failed to find class {}.{}", "System", "Byte"))
        });

        &CLASS_TYPE
    }

    fn class_mut() -> &'static mut Il2CppClass {
        Self::class().clone()
    }
}

/// input: `SomeClass<Arg1, Arg2, ...>`
#[macro_export]
macro_rules! get_generic_class {
    ($name:ident<$($ty:ident),+>) => {
        {
            let class = $name::class();
            unity::il2cpp::class::make_generic(&class, &[$($ty::class()),+])
        }
    };
}

#[skyline::from_offset(0x4503c0)]
pub fn setup_gc_descriptor(class: &Il2CppClass);
