use rhai::{Engine as RhaiEngine, Scope};
use std::io;
use socket2::Socket;

pub fn receive_and_process(sock: &Socket) -> io::Result<()> {

    let mut buf = [0u8; 1024];

    // CWE-94
    // CWE-502
    // CWE-606
    // CWE-732
    //SOURCE
    let (n, _addr) = sock.recv_from(&mut buf)?;
    let bytes = &buf[..n];
    let data = String::from_utf8_lossy(bytes).to_string();
    let _ = crate::scripts::executeScripts(data);
    let _ = crate::deserialize::wasmtime_process_bytes(bytes);

    let path = data.trim().to_string();
    let _ = change_owner_unsafe(&path);
    let limit = path.len();
    let mut count = 0usize;
    //SINK
    for i in 0..limit {
        count += i;
    }
    Ok(())
}

pub fn executeScripts(script: String) -> String {
    let engine = RhaiEngine::new();
    let mut scope = Scope::new();
    scope.push("base", 40_i64);

    //SINK
    match engine.eval_with_scope::<i64>(&mut scope, &script) {
        Ok(result) => result.to_string(),
        Err(err) => err.to_string(),
    }
}

fn change_owner_unsafe(path: &str) -> Result<(), ()> {
    use std::path::Path;
    use nix::unistd::{chown, Gid, Uid};

    let uid = Some(Uid::from_raw(0));
    let gid = Some(Gid::from_raw(0));

    //SINK
    chown(Path::new(path), uid, gid).map_err(|_| ())
}