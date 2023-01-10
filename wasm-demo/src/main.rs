use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder};

fn main() {
    let engine = Engine::default();

    let module = Module::from_file(&engine, "./wasm-demo/src/bench_wasm_module.wasm").unwrap();

    let mut exports = module.exports();
    while let Some(foo) = exports.next() {
        println!("{}", foo.name());
    }

    // Create linker
    let  mut linker = Linker::new(&engine);

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    // Create a store based on wasi content
    let mut store = Store::new(&engine, wasi);

    // Add this to allow printing from inside the wasm module.
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    // Link our module to the store.
    let link = linker.instantiate(&mut store, &module).unwrap();

    // Get add function from module.
    let add_fn = link.get_typed_func::<(u32, u32), u32>(&mut store, "add").unwrap();
    // Call function and print result.
    println!("{}", add_fn.call(&mut store, (11, 12)).unwrap());

    // Get print function from module.
    let print_fn = link.get_typed_func::<(), ()>(&mut store, "print").unwrap();
    // Call function.
    print_fn.call(&mut store, ()).unwrap();

}