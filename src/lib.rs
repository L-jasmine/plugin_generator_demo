use std::ffi::CString;

use wasmedge_plugin_sdk::{
    error::CoreError,
    memory::Memory,
    module::{SyncInstanceRef, SyncModule},
    plugin::{PluginBuilder, PluginDescriptorRef},
    types::{ValType, WasmVal},
};

mod bindings;

pub struct Ctx {
    bars: Vec<usize>,
}

impl Ctx {
    fn insert_bar(&mut self, bar: *mut bindings::Bar) -> usize {
        self.bars.push(bar as usize);
        self.bars.len() - 1
    }
}

pub fn create_module() -> SyncModule<Ctx> {
    fn create_bar<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        _main_memory: &'a mut Memory,
        ctx: &'a mut Ctx,
        args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        let data = if let WasmVal::I32(i) = args[0] {
            i
        } else {
            return Ok(vec![WasmVal::I32(0)]);
        };
        let bar = unsafe { bindings::create_bar(data) };
        let bar_ptr = ctx.insert_bar(bar) as i32;
        Ok(vec![WasmVal::I32(bar_ptr)])
    }

    fn print_foo_bar<'a>(
        _inst_ref: &'a mut SyncInstanceRef,
        main_memory: &'a mut Memory,
        ctx: &'a mut Ctx,
        args: Vec<WasmVal>,
    ) -> Result<Vec<WasmVal>, CoreError> {
        let foo_ptr = if let WasmVal::I32(foo_ptr) = args[0] {
            foo_ptr
        } else {
            return Ok(vec![WasmVal::I32(0)]);
        };

        let foo: &mut bindings::Foo = if let Some(f) =
            main_memory.data_pointer_mut(foo_ptr as usize, std::mem::size_of::<bindings::Foo>())
        {
            unsafe { std::mem::transmute(f.as_mut_ptr()) }
        } else {
            return Ok(vec![WasmVal::I32(0)]);
        };
        let mut foo = *foo;
        let foo_bar = foo.b as usize;
        foo.b = ctx.bars[foo_bar] as *mut bindings::Bar;

        unsafe { bindings::print_foo_bar(foo) };
        Ok(vec![])
    }
    let ctx = Ctx { bars: vec![] };
    let mut module = SyncModule::create("demo", ctx).unwrap();

    module
        .add_func(
            "create_bar",
            (vec![ValType::I32], vec![ValType::I32]),
            create_bar,
        )
        .unwrap();
    module
        .add_func("print_foo_bar", (vec![ValType::I32], vec![]), print_foo_bar)
        .unwrap();

    module
}

#[export_name = "WasmEdge_Plugin_GetDescriptor"]
pub extern "C" fn plugin_hook() -> PluginDescriptorRef {
    let mut builder = PluginBuilder::create(
        CString::new("demo_plugin").unwrap(),
        CString::new("a demo plugin").unwrap(),
    );
    builder.add_module(
        CString::new("demo_module").unwrap(),
        CString::new("a demo of module").unwrap(),
        create_module,
    );

    builder.build()
}
