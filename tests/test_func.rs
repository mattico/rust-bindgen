use support::assert_bind_eq;

#[test]
fn func_ptr() {
    assert_bind_eq(Default::default(), "headers/func_ptr.h", "
        extern \"C\" {
            pub static mut foo: ::std::option::Option<
                extern \"C\" fn(x: ::std::os::raw::c_int,
                              y: ::std::os::raw::c_int) -> ::std::os::raw::c_int>;
        }
    ");
}

#[test]
fn func_ptr_in_struct() {
    assert_bind_eq(Default::default(), "headers/func_ptr_in_struct.h", "
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum baz { TEST = 0, }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Foo {
            pub bar: ::std::option::Option<
                extern \"C\" fn(x: ::std::os::raw::c_int,
                              y: ::std::os::raw::c_int) -> baz>,
        }
        impl ::std::default::Default for Foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn func_proto() {
    assert_bind_eq(Default::default(), "headers/func_proto.h", "
        pub type foo = extern \"C\" fn(bar: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    ");
}

#[test]
fn func_no_proto() {
    assert_bind_eq(Default::default(), "headers/func_no_proto.h", "
        pub type no_proto = extern \"C\" fn() -> ::std::os::raw::c_int;
    ");
}

#[test]
fn with_func_ptr_arg() {
    assert_bind_eq(Default::default(), "headers/func_with_func_ptr_arg.h", "
        pub type ty = extern \"C\" fn() -> ::std::os::raw::c_int;
        extern \"C\" {
            pub fn foo(bar: ::std::option::Option<extern \"C\" fn()>);
            pub fn function(_proc: *mut ty);
        }
    ");
}

#[test]
fn with_array_arg() {
    assert_bind_eq(Default::default(), "headers/func_with_array_arg.h", "
        extern \"C\" {
            pub fn f(x: *mut ::std::os::raw::c_int);
        }
    ");
}
