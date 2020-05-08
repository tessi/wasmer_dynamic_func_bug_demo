use wasmer_runtime::{self as runtime, imports};
use wasmer_runtime_core::{
    import::Namespace, typed_func::DynamicFunc, types::FuncSig, types::Type, types::Value, vm::Ctx,
};
use wasmer_wasi::generate_import_object;

use std::sync::Arc;

fn main() {
    let mut namespace = Namespace::new();
    let signature = Arc::new(FuncSig::new(vec![Type::I32, Type::I32], vec![Type::I32]));
    let func = DynamicFunc::new(
        signature,
        move |_ctx: &mut Ctx, _params: &[Value]| -> Vec<runtime::Value> { vec![Value::from(0)] },
    );
    namespace.insert("random_get", func);
    let mut import_object_overwrites = imports! {};
    import_object_overwrites.register("wasi_snapshot_preview1", namespace);

    let mut import_object = generate_import_object(vec![], vec![], vec![], vec![]);

    // the following line triggers the bug. When overwriting "get_random" with a dynamic func,
    // the errors surface. When inserting the same function with another name in line 16 or commenting
    // out the following line, the program does *not* trigger the bug.
    import_object.extend(import_object_overwrites);

    let wasm_bytes = include_bytes!("wasi_example.wasm");
    let instance = runtime::instantiate(wasm_bytes, &import_object).unwrap();
    instance.call("_start", &vec![]).unwrap();
}
