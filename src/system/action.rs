use std::marker::PhantomData;
use crate::il2cpp::object::Array;
use crate::macro_context::MethodInfo;
use crate::prelude::{Il2CppClass, Il2CppClassData, OptionalMethod};

#[crate::class("System", "Delegate")]
pub struct Delegate<T: Sized + 'static> {
    pub method_ptr: *const u8,
    invoke_impl: *const u8,
    target: Option<&'static T>,
    method: *const MethodInfo,
    __: [u8; 0x38],
}

#[crate::class("System", "MulticastDelegate")]
pub struct MulticastDelegate<T: Sized + 'static> {
    pub parent: DelegateFields<T>,
    pub delegates: &'static Array<&'static mut Delegate<T>>,
}

/// Action that takes no arguments
/// Performs methods of fn(&Obj)
#[crate::class("System", "Action")]
pub struct Action{
    pub method_ptr: *const u8,
    invoke_impl: *const u8,
    pub target_obj: *const u8,  // Reference to Obj
    pub method: Option<&'static MethodInfo>,
}

impl Action{
    pub fn new<T>(target: Option<&T>, method: fn(&T, OptionalMethod)) -> &'static mut Self {
        let mut method_info = MethodInfo::new();
        method_info.method_ptr = method as _;
        method_info.parameters_count = 0;
        let action = Action::instantiate().unwrap();
        action.ctor(target, &method_info);
        action
    }
    pub fn new_from_method_info<T>(target: Option<&T>, method_info: &MethodInfo) -> &'static mut Self {
        let action = Action::instantiate().unwrap();
        action.ctor(target, method_info);
        action
    }
    #[crate::class_method(1)] pub fn invoke(&self); // Offset: 0x33F42B0 Flags: 0
}

/// Action that makes one argument
/// Performs method of fn(&Obj, Arg)
#[crate::class("System", "Action`1")]
pub struct Action1<A>{
    pub method_ptr: *const u8,
    invoke_impl: *const u8,
    pub target_obj: *const u8,  // Reference to Obj
    method: *const MethodInfo,
    __: [u8; 0x38],
    // delegates: &'static Array<&'static mut Delegate<T>>,
    phantom: PhantomData<A>
}

impl<A> Action1<A>
where
    A: Il2CppClassData + Sized,
{
    pub fn new_with_method<T: Il2CppClassData>(object: Option<&T>, method: fn(&T, OptionalMethod)) -> &'static mut Self {
        let action_t_class = Il2CppClass::from_name("System", "Action`1").unwrap();
        let klass = crate::il2cpp::class::make_generic(action_t_class, &[A::class()])
            .expect(format!("Expect Action<{}>", A::class().get_name()).as_str());
        let action = klass.instantiate_as::<Action1<A>>().unwrap();
        let mut method_info = MethodInfo::new();
        method_info.method_ptr = method as _;
        method_info.parameters_count = 1;
        action.ctor(object, &method_info);
        action
    }
    pub fn new<T: Il2CppClassData>(object: Option<&T>, method_info: &MethodInfo) -> &'static mut Self {
        let action_t_class = Il2CppClass::from_name("System", "Action`1").unwrap();
        let klass = crate::il2cpp::class::make_generic(action_t_class, &[A::class()])
            .expect(format!("Expect Action<{}>", A::class().get_name()).as_str());

        let action = klass.instantiate_as::<Action1<A>>().unwrap();
        action.ctor(object, method_info);
        action
    }
    pub fn invoke(&self, arg: A){
        let invoke = unsafe { std::mem::transmute::<_, fn(&Self, A, OptionalMethod)>(self.klass.get_methods()[1].method_ptr) };
        invoke(self, arg, None);
    }
}
/// Action that takes no arguments
/// Performs methods of fn(&Obj) -> R
#[crate::class("System", "Func`1")]
pub struct Func<T: 'static , R>{
    pub method_ptr: *const u8,
    invoke_impl: *const u8,
    pub target: Option<&'static T>,
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: &'static Array<&'static mut Delegate<T>>,
    phantom: PhantomData<R>,
}
impl<T, R> Func<T, R>
where
    T: Il2CppClassData + Sized,
    R: Il2CppClassData + Sized + 'static,
{
    pub fn new(object: Option<&T>, method_info: &MethodInfo) -> &'static mut Self {
        let func_r_class = Il2CppClass::from_name("System", "Func`1").unwrap();
        let klass = crate::il2cpp::class::make_generic(func_r_class, &[R::class()])
            .expect(format!("Expect Func<{}>", R::class().get_name()).as_str());

        let action = klass.instantiate_as::<Func<T, R>>().unwrap();
        action.ctor(object, method_info);
        action
    }
    pub fn invoke(&self) -> R {
        let invoke = unsafe { std::mem::transmute::<_, fn(&Self, OptionalMethod) -> R >(self.klass.get_methods()[1].method_ptr) };
        invoke(self, None)
    }
}

/// Action that takes no arguments
/// Performs methods of fn(&Obj, A) -> R
#[crate::class("System", "Func`2")]
pub struct Func1<T: 'static, A, R>{
    pub method_ptr: *const u8,
    invoke_impl: *const u8,
    pub target: Option<&'static T>,
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: &'static Array<&'static mut Delegate<T>>,
    phantom1: PhantomData<A>,
    phantom2: PhantomData<R>,
}

/// Trait to use the .ctor for all delegate classes
pub trait SystemDelegate {
    fn ctor<T: Il2CppClassData + Sized>(&self, object: Option<&T>, method_info: &MethodInfo) {
        unsafe { system_action_ctor(self, object, method_info); }
    }
}
impl<T> SystemDelegate for Delegate<T> {}
impl<T> SystemDelegate for MulticastDelegate<T> {}
impl SystemDelegate for Action {}
impl<A> SystemDelegate for Action1<A> {}
impl<T,R> SystemDelegate for Func<T,R> {}

#[crate::from_offset("System", "Action", ".ctor")]
fn system_action_ctor<D, T>(this: &D, obj: Option<&T>, method_info: &MethodInfo)
where
    D: SystemDelegate,
    T: Il2CppClassData + Sized;
