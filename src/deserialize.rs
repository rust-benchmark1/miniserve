use wasmtime::{Engine, Module};

#[derive(Debug)]
pub enum Status {
    BadRequest,
    InternalError,
}

pub fn wasmtime_process_bytes(bytes: &[u8]) -> Result<Module, Status> {
    let engine = Engine::default();

    //SINK
    let module = unsafe {
        Module::deserialize_raw(&engine, bytes)
    }
    .map_err(|_| Status::BadRequest)?;

    Ok(module)
}
