use connection::HasSocket;
mod connection;

fn main() {
    let mut client = connection::client::ClientConnection { socket: connection::client::client_connect("173.230.156.174", 80) };
    client.send("GET / HTTP/1.1\r\nHost: 173.230.156.174\r\n\r\n".to_string());
    println!("{}", client.recv());
}
