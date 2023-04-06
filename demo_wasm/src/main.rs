#[link(wasm_import_module = "demo")]
extern "C" {
    fn create_bar(idata: i32) -> i32;
    fn print_foo_bar(foo_ptr: i32);
}

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

fn main() {
    let bar = unsafe { create_bar(2) as *mut Bar };
    let foo = Foo { a: 1, b: bar };
    unsafe { print_foo_bar(&foo as *const Foo as i32) };
    println!("Hello, world!");
}
