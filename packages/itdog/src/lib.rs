mod req;

use std::sync::Arc;

use fastwebsockets::handshake;
use fastwebsockets::FragmentCollector;
use fastwebsockets::Frame;
use reqwest::header::CONNECTION;
use reqwest::header::HOST;
use reqwest::header::SEC_WEBSOCKET_KEY;
use reqwest::header::SEC_WEBSOCKET_VERSION;
use reqwest::header::UPGRADE;
use rustls::ClientConfig;
use rustls::RootCertStore;
use std::future::Future;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::Receiver;
use tokio_rustls::TlsConnector;
use tracing::debug;
use tracing::trace;

pub const DEFAULT_KEY: &str = "token_20230313000136kwyktxb0tgspm00yo5";

pub use req::Message;
pub use req::ReqClient;

#[derive(Debug)]
pub struct ItdogClient<'a> {
    key: &'a str,

    host: &'a str,

    cancell: Receiver<bool>,

    respone: Sender<req::Message>,
}

macro_rules! return_if_cancell {
    ($_self:ident) => {
        if let Ok(flag) = $_self.cancell.try_recv() {
            if flag {
                return Ok(());
            }
        }
    };
}

impl<'a> ItdogClient<'a> {
    pub fn new(
        key: &'a str,
        host: &'a str,
        cancell: Receiver<bool>,
        respone: Sender<req::Message>,
    ) -> Self {
        Self {
            key,
            host,
            cancell,
            respone,
        }
    }

    pub async fn query(&mut self) -> color_eyre::Result<()> {
        let cli = reqwest::ClientBuilder::new().cookie_store(true).build()?;
        let server_host = "www.itdog.cn";

        debug!("try to httping host `{}`", self.host);
        return_if_cancell!(self);

        let reqc = req::ReqClient::new(cli, self.key, self.host);
        let pingmsg = reqc.req_wssocket_msg("https://www.itdog.cn/http/").await?;

        debug!("construct ping message `{pingmsg}`");
        return_if_cancell!(self);

        // Prepare a tls connection
        let tcp_stream = TcpStream::connect(&format!("{}:443", server_host)).await?;
        let config = ClientConfig::builder()
            .with_root_certificates(RootCertStore::from_iter(
                webpki_roots::TLS_SERVER_ROOTS.iter().cloned(),
            ))
            .with_no_client_auth();
        let tls_connector = TlsConnector::from(Arc::new(config));

        debug!("construct tls connector");
        return_if_cancell!(self);

        let server_name =
            tokio_rustls::rustls::pki_types::ServerName::try_from(server_host)?.to_owned();
        let tls_stream = tls_connector.connect(server_name, tcp_stream).await?;

        // Prepare a request
        let request = reqc
            .cli()
            .get("wss://www.itdog.cn/websockets")
            .header(HOST, server_host)
            .header(UPGRADE, "websocket")
            .header(CONNECTION, "upgrade")
            .header(SEC_WEBSOCKET_KEY, fastwebsockets::handshake::generate_key())
            .header(SEC_WEBSOCKET_VERSION, "13")
            .build()?;
        let request: http::Request<reqwest::Body> = request.try_into()?;
        let (parts, _) = request.into_parts();
        let request = http::Request::from_parts(parts, String::default());

        debug!("construct http request: `{request:?}`");
        return_if_cancell!(self);

        let (websocket, _) = handshake::client(&SpawnExecutor, request, tls_stream).await?;
        let mut websocket = FragmentCollector::new(websocket);

        debug!("sending payload message to websocket");
        websocket
            .write_frame(Frame::text(fastwebsockets::Payload::Borrowed(
                pingmsg.as_bytes(),
            )))
            .await?;

        debug!("waiting for server reply..");
        return_if_cancell!(self);

        let mut count = 0;

        // The WebSocket is also a `TryStream` over `Message`s.
        while let Ok(message) = websocket.read_frame().await {
            match message.opcode {
                fastwebsockets::OpCode::Text => {
                    let text = String::from_utf8(message.payload.to_vec())?;

                    count += 1;
                    trace!("got text message {count}: {}", text);

                    if text.contains("\"type\":\"finished\"") {
                        break;
                    } else {
                        self.respone.send(serde_json::from_str(&text)?).await?;
                    }
                }
                fastwebsockets::OpCode::Close => {
                    break;
                }
                fastwebsockets::OpCode::Continuation => {
                    println!("........?");
                }
                fastwebsockets::OpCode::Binary => {
                    println!("...........?");
                }
                _ => {
                    println!("..................................");
                }
            }
            return_if_cancell!(self);
        }

        debug!("sending close message to websocket");

        websocket
            .write_frame(Frame::close_raw(vec![].into()))
            .await?;

        debug!("done! received text message count = {count}");

        Ok(())
    }
}

struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}
