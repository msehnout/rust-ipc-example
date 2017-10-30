use std::os::unix::net::UnixListener;
use std::io::{self, Read, Error, ErrorKind};
use std::str;

static SOCK_PATH: &str = "/tmp/rusty-server";
static BUF_SIZE: usize = 512;

fn run_server() -> io::Result<()> {
    let _ = std::fs::remove_file(SOCK_PATH);
    let listener = UnixListener::bind(SOCK_PATH)?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = vec![0u8; BUF_SIZE];
    loop {
        match stream.read(&mut buffer){
            Ok(0) => break,
            Ok(len) => println!("Received: {}", str::from_utf8(&buffer[..len])
                                .map_err(|_| Error::new(ErrorKind::Other, "Input is not valid UTF-8"))?
                                .trim()),
            Err(e) => return Err(e),
        }
    }
    drop(listener);
    std::fs::remove_file(SOCK_PATH)?;
    Ok(())
}

fn main() {
    println!("Unix domain socket server:");
    if let Err(e) = run_server() {
        eprint!("Server returned an error: {:?}", e);
    }
}
