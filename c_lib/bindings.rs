/* automatically generated by rust-bindgen 0.64.0 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub a: ::std::os::raw::c_int,
    pub b: *mut Bar,
}
extern "C" {
    pub fn create_bar(data: ::std::os::raw::c_int) -> *mut Bar;
}
extern "C" {
    pub fn print_foo_bar(foo: Foo);
}
