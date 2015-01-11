use std::io::TcpStream;

trait HasSocket {
	fn send(&mut self, String);
	fn recv(&mut self) -> String;
}

struct ClientConnection {
	socket: TcpStream,
}

/// Establish and return a client connection with a supplied host and
/// port.
fn client_connect(host: &str, port: u32) -> TcpStream {
	TcpStream::connect(format!("{}:{}", host, port).as_slice()).unwrap()
}

impl HasSocket for ClientConnection {
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

struct ServerConnection {
	socket: TcpStream,
}

//TODO: Add server socket generator

impl HasSocket for ServerConnection {
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

fn main() {
	let mut client = ClientConnection { socket: client_connect("173.230.156.174", 80) };
	client.send("GET / HTTP/1.1\r\nHost: 173.230.156.174\r\n\r\n".to_string());
	println!("{}", client.recv());
}
