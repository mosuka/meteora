use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};

fn main() -> Result<(), std::io::Error> {
    let mut proc = Command::new("./protoc_rust.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run");
    let ret = proc.wait().unwrap();

    if ret.success() {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("failed to run: {}", ret.code().unwrap()),
        ))
    }
}
