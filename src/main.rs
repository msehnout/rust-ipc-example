use std::os::unix::net::UnixListener;
use std::io::{self, Read, Error, ErrorKind};
use std::str;

fn run_server() -> io::Result<()> {
    let _ = std::fs::remove_file("/tmp/rusty-server");
    let listener = UnixListener::bind("/tmp/rusty-server")?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = vec![0u8; 512];
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
    std::fs::remove_file("/tmp/rusty-server")?;
    Ok(())
}

fn main() {
    println!("Unix domain socket server:");
    if let Err(e) = run_server() {
        eprint!("Server returned an error: {:?}", e);
    }
}
