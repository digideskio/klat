use std::io::{TcpStream, TcpListener, Acceptor, Listener};
use std::thread::Thread;
pub use connection;

pub struct ServerConnection {
	pub socket: TcpStream,
}

/// Called from non-main thread on an incoming connection. Handles all
/// incoming/outcoming data and server logic.
fn server_client_handler(conn: ServerConnection) {
	() //TODO: Write client  handler
}

/// Create a listener and run mainloop, spawning additional threads per
/// incoming connection.
pub fn server_mainloop(host: &str, port: u32) {
	let listener = TcpListener::bind(format!("{}:{}", host, port).as_slice());
	
	let mut acceptor = listener.listen();
	for stream in acceptor.incoming() {
		match stream {
			Ok(stream) => {
				Thread::spawn(move || {
					server_client_handler(ServerConnection {
						socket: stream
						});
					});
				},
			Err(e) => (), //TODO: Add error reports [connection failed]
		}
	}
	
	drop(acceptor);
}

impl connection::HasSocket for ServerConnection {
	fn send(&mut self, msg: String) {
		&self.socket.write(msg.as_bytes());
	}
	
	fn recv(&mut self) -> String {
		match self.socket.read_to_end() {
			Ok(s) => String::from_utf8(s).unwrap(),
			Err(_) => "Response could not be read.".to_string(),
		}
	}
}
