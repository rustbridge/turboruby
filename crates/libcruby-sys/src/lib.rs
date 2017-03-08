#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libc;

use std::ffi::CString;

pub const RUNTIME_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn check_version() {
    // All this to look up `HelixRuntime::VERSION`!
    let runtime_name = unsafe { rb_intern(CString::new("HelixRuntime").unwrap().as_ptr()) };
    let version_name = unsafe { rb_intern(CString::new("VERSION").unwrap().as_ptr()) };
    let rb_HelixRuntime = unsafe { rb_const_get(rb_cObject, runtime_name) };
    let rb_VERSION = unsafe { rb_const_get(rb_HelixRuntime, version_name) };
    let size = unsafe { RSTRING_LEN(rb_VERSION) };
    let ptr = unsafe { RSTRING_PTR(rb_VERSION) };
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
    let version = unsafe { std::str::from_utf8_unchecked(slice) };

    if RUNTIME_VERSION != version {
        panic!("Helix Rust runtime version ({}) doesn't match Ruby runtime version ({}).", RUNTIME_VERSION, version);
    }
}

pub type void_ptr = *const libc::c_void;
pub type c_string = *const libc::c_char;
// pub type c_func = extern "C" fn(...);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ID(void_ptr);

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct VALUE(void_ptr);

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RubyException(isize);

impl RubyException {
    pub fn new() -> RubyException {
        RubyException(0)
    }

    pub fn empty() -> RubyException {
        RubyException(0)
    }

    pub fn for_tag(tag: isize) -> RubyException {
        RubyException(tag)
    }
}

pub const EMPTY_EXCEPTION: RubyException = RubyException(0);

#[cfg_attr(windows, link(name="helix-runtime"))]
extern "C" {
    #[link_name = "HELIX_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "HELIX_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "HELIX_Qnil"]
    pub static Qnil: VALUE;

    #[link_name = "rb_cObject"]
    pub static rb_cObject: VALUE;

    #[link_name = "HELIX_RSTRING_LEN"]
    pub fn RSTRING_LEN(string: VALUE) -> isize;

    #[link_name = "HELIX_RSTRING_PTR"]
    pub fn RSTRING_PTR(string: VALUE) -> c_string;

    #[link_name = "HELIX_rb_utf8_str_new"]
    pub fn rb_utf8_str_new(string: c_string, len: libc::c_long) -> VALUE;

    #[link_name = "HELIX_RARRAY_LEN"]
    pub fn RARRAY_LEN(array: VALUE) -> isize;

    #[link_name = "HELIX_RARRAY_PTR"]
    pub fn RARRAY_PTR(array: VALUE) -> void_ptr;

    #[link_name = "HELIX_RB_TYPE_P"]
    pub fn RB_TYPE_P(val: VALUE, rb_type: isize) -> bool;

    #[link_name = "HELIX_TYPE"]
    pub fn TYPE(val: VALUE) -> isize;

    pub fn rb_check_type(v: VALUE, rb_type: isize);

    #[link_name = "HELIX_NUM2U64"]
    pub fn NUM2U64(v: VALUE) -> u64;

    #[link_name = "HELIX_U642NUM"]
    pub fn U642NUM(num: u64) -> VALUE;

    #[link_name = "HELIX_NUM2I64"]
    pub fn NUM2I64(v: VALUE) -> i64;

    #[link_name = "HELIX_I642NUM"]
    pub fn I642NUM(num: i64) -> VALUE;

    #[link_name = "HELIX_T_STRING"]
    pub static T_STRING: isize;

    #[link_name = "HELIX_T_ARRAY"]
    pub static T_ARRAY: isize;

    #[link_name = "HELIX_T_TRUE"]
    pub static T_TRUE: isize;

    #[link_name = "HELIX_T_FALSE"]
    pub static T_FALSE: isize;

    #[link_name = "HELIX_T_FIXNUM"]
    pub static T_FIXNUM: isize;

    #[link_name = "HELIX_T_BIGNUM"]
    pub static T_BIGNUM: isize;

    // unknown if working?
    // fn rb_define_variable(name: c_string, value: *const VALUE);
    pub fn rb_obj_class(obj: VALUE) -> VALUE;
    pub fn rb_obj_classname(obj: VALUE) -> c_string;
    pub fn rb_const_get(class: VALUE, name: ID) -> VALUE;
    pub fn rb_define_global_const(name: c_string, value: VALUE);
    pub fn rb_define_module(name: c_string) -> VALUE;
    pub fn rb_define_module_under(namespace: VALUE, name: c_string) -> VALUE;
    pub fn rb_define_class(name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_class_under(namespace: VALUE, name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_alloc_func(klass: VALUE, func: extern "C" fn(klass: VALUE) -> VALUE);
    pub fn rb_define_method(class: VALUE, name: c_string, func: void_ptr, arity: isize);
    pub fn rb_inspect(value: VALUE) -> VALUE;
    pub fn rb_intern(string: c_string) -> ID;
    pub fn rb_jump_tag(state: RubyException) -> !;
    pub fn rb_protect(try: extern "C" fn(v: void_ptr) -> VALUE,
                      arg: void_ptr,
                      state: *mut RubyException)
                      -> VALUE;

    #[link_name = "HELIX_Data_Wrap_Struct"]
    pub fn Data_Wrap_Struct(klass: VALUE, mark: extern "C" fn(void_ptr), free: extern "C" fn(void_ptr), data: void_ptr) -> VALUE;

    #[link_name = "HELIX_Data_Get_Struct_Value"]
    pub fn Data_Get_Struct_Value(obj: VALUE) -> void_ptr;

    #[link_name = "HELIX_Data_Set_Struct_Value"]
    pub fn Data_Set_Struct_Value(obj: VALUE, data: void_ptr);
}
