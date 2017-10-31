use std::os::unix::net::UnixListener;
use std::io::{self, Read};
use std::str;

static SOCK_PATH: &str = "/tmp/rusty-server";
static BUF_SIZE: usize = 512;

// Run server and return success or error
fn run_server() -> io::Result<()> {
    // Unlink existing socket
    let _ = std::fs::remove_file(SOCK_PATH);
    // Bind a new listener to the path
    let listener = UnixListener::bind(SOCK_PATH)?;
    // Accept one connection yielding an input stream
    let (mut stream, _) = listener.accept()?;
    // Create a read buffer
    let mut buffer = vec![0u8; BUF_SIZE];
    // Loop forever
    loop {
        // Read from the input stream as long as there are some data
        match stream.read(&mut buffer){
            // This is the end of the stream, break the loop
            Ok(0) => break,
            // Convert the received data as UTF-8 encoded text. Trim leading and trailing whitespaces.
            Ok(len) => println!("Received: {}", str::from_utf8(&buffer[..len])
                                .unwrap_or("Non-valid UTF8.")
                                .trim()),
            // In case of error, return from the function
            Err(e) => return Err(e),
        }
    }
    // Explicitly drop listener
    drop(listener);
    // Again, unlink the socket
    std::fs::remove_file(SOCK_PATH)?;
    // Return success
    Ok(())
}

fn main() {
    println!("Unix domain socket server:");
    // Catch any errors, that might occur
    if let Err(e) = run_server() {
        eprint!("Server returned an error: {:?}", e);
    }
}
