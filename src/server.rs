use crate::{codec::BitcodeCodec, request::Request};
use futures::{SinkExt, StreamExt};
use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions,
    tokio::{Listener, Stream, prelude::*},
};
use std::io;
use tokio_util::codec::Framed;

const LISTENER_SOCKET_NAME: &str = "sql_rs.sock";

pub struct Server {
    listener: Listener,
}

impl Server {
    pub fn build() -> io::Result<Self> {
        let listener = Self::create_socket(LISTENER_SOCKET_NAME)?;

        Ok(Self { listener })
    }

    pub async fn serve(&self) -> io::Result<()> {
        loop {
            let connection = match self.listener.accept().await {
                Ok(c) => c,
                Err(_) => continue,
            };

            tokio::spawn(
                async move { if let Err(_) = Self::handle_connection(connection).await {} },
            );
        }
    }

    async fn handle_connection(connection: Stream) -> io::Result<()> {
        let mut framed = Framed::new(connection, BitcodeCodec::<Request>::new());

        while let Some(request) = framed.next().await {
            match request? {
                Request::Shutdown => framed.send(Request::Shutdown).await?,
            }
        }

        Ok(())
    }

    fn create_socket(socket_name: &str) -> io::Result<Listener> {
        let name = socket_name.to_ns_name::<GenericNamespaced>()?;
        let opts = ListenerOptions::default().name(name);

        opts.create_tokio()
    }
}
