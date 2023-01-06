use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::str::{self};

use anyhow::Result;
use async_trait::async_trait;
use russh::*;
use russh::client::Msg;
use russh_keys::*;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = anyhow::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), anyhow::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), anyhow::Error>>;
 
    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }
    fn check_server_key(self, _server_public_key: &key::PublicKey) -> Self::FutureBool {
        self.finished_bool(true)
    }
    /*fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
        self.finished(session)
    }
    fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
        self.stdout.send(data.to_vec());
        self.finished(session)
    }*/
 }

pub struct Session {
    session: client::Handle<Client>,
    stdout: mpsc::Sender<Vec<u8>>,
}

impl Session {
    pub async fn connect<P: AsRef<Path>>(
        key_path: P,
        user: impl Into<String>,
        addrs: std::net::SocketAddr,
        stdout: mpsc::Sender<Vec<u8>>,
    ) -> Result<Self> {
        let key_pair = load_secret_key(key_path, None)?;
        let config = client::Config {
            connection_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let sh = Client {};
        let mut session = client::connect(config, addrs, sh).await?;
        let _auth_res = session
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;
        Ok(Self { session, stdout })
    }

    pub async fn run(&mut self, command: &str, stdin: mpsc::Receiver<Vec<u8>>, ) -> Result<()> {
        let mut channel: Arc<Mutex<Channel<Msg>>> = Arc::new(Mutex::new(self.session.channel_open_session().await.unwrap()));
        
        let guard = channel.lock().unwrap();
        guard.exec(false, command).await?;

        let mut in_stream: ReceiverStream<Vec<u8>> = ReceiverStream::new(stdin);
        tokio::spawn(async move {
            while let Some(item) = in_stream.next().await {
                let guard = channel.get_mut().unwrap();
                match str::from_utf8(&item) {
                    Ok(v) => guard.exec(false, v).await.unwrap(),
                    Err(_) => {/* Not handled :) */},
                };
            }
        });

        while let Some(msg) = channel.get_mut().unwrap().wait().await {
            match msg {
                russh::ChannelMsg::Data { ref data } => {
                    let output = Vec::new();
                    output.write_all(data).unwrap();
                    self.stdout.send(output).await.unwrap();
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
