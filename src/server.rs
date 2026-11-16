use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions,
    tokio::{Listener, prelude::*},
};
use std::io;

pub struct Server {}

impl Server {
    fn create_socket(socket_name: &str) -> io::Result<Listener> {
        let name = socket_name.to_ns_name::<GenericNamespaced>()?;
        let opts = ListenerOptions::default();

        opts.create_tokio()
    }
}
