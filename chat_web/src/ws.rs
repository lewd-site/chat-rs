use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::protocol::{Message, WebSocket};

type WsClient = Arc<Mutex<WebSocket<TcpStream>>>;
type WsClients = Arc<Mutex<HashMap<SocketAddr, WsClient>>>;

pub struct Ws {
    clients: WsClients,
}

impl Ws {
    fn handle_connection(clients: &WsClients, addr: SocketAddr, stream: TcpStream) {
        let websocket = tungstenite::accept(stream).unwrap();
        let client = Arc::new(Mutex::new(websocket));

        Arc::clone(&clients)
            .lock()
            .unwrap()
            .insert(addr, Arc::clone(&client));
    }

    pub fn new(address: &str) -> Ws {
        let clients: WsClients = Arc::new(Mutex::new(HashMap::new()));

        let thread_address = String::from(address);
        let thread_clients = Arc::clone(&clients);
        thread::spawn(move || loop {
            let listener = TcpListener::bind(&thread_address).unwrap();
            let (stream, addr) = listener.accept().unwrap();
            Ws::handle_connection(&thread_clients, addr, stream);
        });

        Ws { clients }
    }

    pub fn send_to_all(&self, message: &str) {
        let message = Message::text(message);
        let clients = Arc::clone(&self.clients);
        thread::spawn(move || {
            let mut clients = clients.lock().unwrap();
            for (key, value) in clients.clone().iter() {
                let mut client = value.lock().unwrap();
                match client.write_message(message.clone()) {
                    Err(_) => clients.remove(key),
                    _ => None,
                };
            }
        });
    }
}
