pub mod client;

pub mod server;

pub trait HasSocket {
	fn send(&mut self, String);
	fn recv(&mut self) -> String;
}
