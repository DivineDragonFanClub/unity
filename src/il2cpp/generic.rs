use crate::il2cpp::Il2CppType;
use crate::macro_context::MethodInfo;

#[repr(C)]
#[derive(Clone, Copy)]
pub union GenericMethodInfo {
    pub generic_method: &'static Il2CppGenericMethod,
    pub generic_container: Il2CppGenericContainer,
}
#[repr(C)]
pub struct Il2CppGenericMethod {
    pub method_definition: &'static mut MethodInfo,
    pub generic_context: Il2CppGenericContext,
}

#[repr(C)]
pub struct Il2CppGenericContext {
    pub class_inst: Option<&'static Il2CppGenericInst>,
    pub method_inst: Option<&'static Il2CppGenericInst>,
}

#[repr(C)]
pub struct Il2CppGenericInst {
    pub type_argc: u32,
    pub type_argv: *const &'static Il2CppType,
}
impl Il2CppGenericInst {
    pub fn get_types(&self) -> &[&'static Il2CppType] {
        unsafe { std::slice::from_raw_parts(self.type_argv, self.type_argc as _) }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Il2CppGenericContainer {
    pub owner: i32,
    pub type_argc: i32,
    pub is_method: i32,
    pub generic_parameter_start: i32,
}

pub fn create_generic_method_info(method_info: &MethodInfo, types: &[&Il2CppType]) -> &'static MethodInfo {
    let len = types.len();
    let ty = types.as_ptr();
    let generic_method_inst = unsafe { create_generic_inst(ty, len) };
    let generic_class_inst: Option<&Il2CppGenericInst> =
        if method_info.bitflags & 2 == 0 { None }
        else {
            method_info.class.and_then(|klass|klass._1.generic_class)
                .and_then(|generic_class| generic_class.context.class_inst)
        };
    let generic_method = unsafe { create_generic_method(method_info, generic_class_inst, Some(generic_method_inst)) };
    let method = unsafe { generic_method_create_method_info(generic_method, 0) };
    method
}


/// Macro to generate an instantiated generic MethodInfo
/// input: `MethodInfo<Class1, Class2, ...>`
#[macro_export]
macro_rules! get_generic_method {
    ($method:ident<$($ty:ident),+>) => {
        unity::il2cpp::generic::create_generic_method_info($method, &[$($ty::class().get_type()),+])
    };
}

#[skyline::from_offset(0x43e2bc)]
fn create_generic_inst(types: *const &Il2CppType, len: usize) -> &'static Il2CppGenericInst;

#[skyline::from_offset(0x47c0d4)]
fn generic_method_create_method_info(generic_method: &Il2CppGenericMethod, flags: i32) -> &'static MethodInfo;

#[skyline::from_offset(0x439a48)]
fn create_generic_method(
    method_definition: &MethodInfo,
    generic_class_inst: Option<&Il2CppGenericInst>,
    generic_method_inst: Option<&Il2CppGenericInst>
) -> &'static Il2CppGenericMethod;
