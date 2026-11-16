use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions,
    tokio::{Listener, prelude::*},
};
use std::io;

const LISTENER_SOCKET_NAME: &str = "sql_rs.sock";

pub struct Server {
    listener: Listener,
}

impl Server {
    pub fn build() -> io::Result<Self> {
        let listener = Self::create_socket(LISTENER_SOCKET_NAME)?;

        Ok(Self { listener })
    }

    pub fn serve(&self) -> io::Result<()> {
        loop {
            let connection = match self.listener.accept() {
                Ok(c) => c,
                Err(_) => continue,
            };
        }
    }

    fn create_socket(socket_name: &str) -> io::Result<Listener> {
        let name = socket_name.to_ns_name::<GenericNamespaced>()?;
        let opts = ListenerOptions::default();

        opts.create_tokio()
    }
}
