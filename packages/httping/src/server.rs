use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

mod itdog;

pub use itdog::Itdog;

#[async_trait::async_trait]
pub trait PingServer {
    fn name(&self) -> &str;

    async fn ping(
        &self,
        host: String,
        flag: oneshot::Receiver<bool>,
        out: mpsc::Sender<Option<TaskRespone>>,
    ) -> color_eyre::Result<()>;
}

#[derive(Debug)]
pub struct Task {
    host: String,
    handler: Option<JoinHandle<color_eyre::Result<()>>>,
    resp: Vec<TaskRespone>,

    resp_rx: mpsc::Receiver<Option<TaskRespone>>,
    cancell: Option<oneshot::Sender<bool>>,
    ending: bool,
}

impl Task {
    pub fn new(
        host: String,
        handler: JoinHandle<color_eyre::Result<()>>,
        cancell: oneshot::Sender<bool>,
        respone: mpsc::Receiver<Option<TaskRespone>>,
    ) -> Self {
        Self {
            host,
            handler: Some(handler),
            resp: vec![],
            cancell: Some(cancell),
            resp_rx: respone,
            ending: false,
        }
    }

    pub fn host(&self) -> &str {
        self.host.as_str()
    }

    pub fn respone(&self) -> &[TaskRespone] {
        self.resp.as_slice()
    }

    pub fn try_cancell(&mut self) -> color_eyre::Result<()> {
        if let Some(tx) = self.cancell.take() {
            tx.send(true)
                .map_err(|_| color_eyre::eyre::eyre!("cancell failed"))
        } else {
            Ok(())
        }
    }

    pub fn recv_respone(&mut self) {
        if !self.ending {
            let ret = self.resp_rx.try_recv();

            match ret {
                Ok(Some(resp)) => {
                    self.resp.push(resp);
                }
                Ok(None) => {
                    self.ending = true;
                }
                Err(_) => {}
            }
        }
    }

    pub fn take_handler(&mut self) -> Option<JoinHandle<color_eyre::Result<()>>> {
        self.handler.take()
    }

    pub fn ending(&self) -> bool {
        self.ending
    }
}

#[derive(Debug, Default, Clone)]
pub struct TaskRespone {
    loc: String,

    ip: String,

    status: i32,

    total_cost: String,

    other_name_list: Vec<String>,

    other_cost_list: Vec<String>,

    redirect: i32,

    redirect_cost: String,
}

impl TaskRespone {
    pub fn with_loc(mut self, loc: String) -> Self {
        self.loc = loc;
        self
    }

    pub fn with_ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }

    pub fn with_status(mut self, status: i32) -> Self {
        self.status = status;
        self
    }

    pub fn with_total_cost(mut self, total_cost: String) -> Self {
        self.total_cost = total_cost;
        self
    }

    pub fn with_other_name_list(mut self, other_name_list: Vec<String>) -> Self {
        self.other_name_list = other_name_list;
        self
    }

    pub fn with_other_cost_list(mut self, other_cost_list: Vec<String>) -> Self {
        self.other_cost_list = other_cost_list;
        self
    }

    pub fn with_redirect(mut self, redirect: i32) -> Self {
        self.redirect = redirect;
        self
    }

    pub fn with_redirect_cost(mut self, redirect_cost: String) -> Self {
        self.redirect_cost = redirect_cost;
        self
    }

    pub fn loc(&self) -> &str {
        self.loc.as_str()
    }

    pub fn ip(&self) -> &str {
        self.ip.as_str()
    }

    pub fn status(&self) -> i32 {
        self.status
    }

    pub fn total_cost(&self) -> &str {
        self.total_cost.as_str()
    }

    pub fn other_name_list(&self) -> &[String] {
        self.other_name_list.as_slice()
    }

    pub fn other_cost_list(&self) -> &[String] {
        self.other_cost_list.as_slice()
    }

    pub fn redirect(&self) -> i32 {
        self.redirect
    }

    pub fn redirect_cost(&self) -> &str {
        self.redirect_cost.as_str()
    }

    pub fn set_loc(&mut self, loc: String) -> &mut Self {
        self.loc = loc;
        self
    }

    pub fn set_ip(&mut self, ip: String) -> &mut Self {
        self.ip = ip;
        self
    }

    pub fn set_status(&mut self, status: i32) -> &mut Self {
        self.status = status;
        self
    }

    pub fn set_total_cost(&mut self, total_cost: String) -> &mut Self {
        self.total_cost = total_cost;
        self
    }

    pub fn set_other_name_list(&mut self, other_name_list: Vec<String>) -> &mut Self {
        self.other_name_list = other_name_list;
        self
    }

    pub fn set_other_cost_list(&mut self, other_cost_list: Vec<String>) -> &mut Self {
        self.other_cost_list = other_cost_list;
        self
    }

    pub fn set_redirect(&mut self, redirect: i32) -> &mut Self {
        self.redirect = redirect;
        self
    }

    pub fn set_redirect_cost(&mut self, redirect_cost: String) -> &mut Self {
        self.redirect_cost = redirect_cost;
        self
    }
}
