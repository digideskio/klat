use std::io::{TcpStream, TcpListener, Acceptor, Listener};
use std::thread::Thread;

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

/// Called from non-main thread on an incoming connection. Handles all
/// incoming/outcoming data and server logic.
fn server_client_handler(conn: ServerConnection) {
	() //TODO: Write client handler
}

/// Create a listener and run mainloop, spawning additional threads per
/// incoming connection.
fn server_mainloop(host: &str, port: u32) {
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
