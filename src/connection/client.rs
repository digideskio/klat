use std::io::{TcpStream, TcpListener, Acceptor, Listener};
use connection;

pub struct ClientConnection {
	pub socket: TcpStream,
}

/// Establish and return a client connection with a supplied host and
/// port.
pub fn client_connect(host: &str, port: u32) -> TcpStream {
	TcpStream::connect(format!("{}:{}", host, port).as_slice()).unwrap()
}

impl connection::HasSocket for ClientConnection {
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
