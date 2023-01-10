# WASM Module example benchmark

Trying to identify the overhead of a WASM module when used to export some functionality. 

## Structure

* `wasm-module`: a simple WASM module exporting some simple functions
* `wasm-demo`: a dummy application that loads the WASM module and calls the simple functions. 

## Steps to run

### Build the module

```shell
cd wasm-module
cargo build --target wasm32-wasi --release
```

This builds a WASM module under `target/wasm32-wasi/release/wasm_module.wasm`, 
we need to make it available to the dummy application by copying it.

From the top level directory.

```shell
cp target/wasm32-wasi/release/wasm_module.wasm wasm-demo/src/bench_wasm_module.wasm
```

###  Build the dummy application

```shell
cargo build
```

###  Run the dummy application

From the top level directory.

```shell
cargo run
```

Output:
```shell
memory
add
print
23
Hello from wasm-module!
```

## Next Steps:

* Evaluate the overhead of module call 
* Investigate various function parameter types overhead
