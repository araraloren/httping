use itdog::ItdogClient;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::Receiver;
use tracing::{debug, trace};

use super::{PingServer, TaskRespone};

pub struct Itdog;

#[async_trait::async_trait]
impl PingServer for Itdog {
    fn name(&self) -> &str {
        "itdog"
    }

    async fn ping(
        &self,
        host: String,
        cancell: Receiver<bool>,
        resp: Sender<Option<TaskRespone>>,
    ) -> color_eyre::Result<()> {
        let (send, mut recv) = tokio::sync::mpsc::channel(128);

        debug!("start ping request for `{host}`");

        tokio::spawn(async move {
            let mut itdog = ItdogClient::new(itdog::DEFAULT_KEY, &host, cancell, send);
            itdog.query().await.unwrap();
        });

        while let Some(msg) = recv.recv().await {
            let task_resp = TaskRespone::default()
                .with_loc(msg.name().to_string())
                .with_ip(msg.ip().to_string())
                .with_status(msg.http_code())
                .with_redirect(msg.redirect())
                .with_redirect_cost(msg.redirect_time().to_string())
                .with_total_cost(msg.all_time().to_string())
                .with_other_name_list(
                    ["DNS时间", "连接时间", "下载时间"]
                        .map(String::from)
                        .to_vec(),
                )
                .with_other_cost_list(
                    [msg.dns_time(), msg.connect_time(), msg.download_time()]
                        .map(String::from)
                        .to_vec(),
                );

            trace!(
                "sending respone ip = `{}`, status = `{}`",
                msg.ip(),
                msg.http_code()
            );

            resp.send(Some(task_resp)).await?;
        }

        resp.send(None).await?;

        Ok(())
    }
}
