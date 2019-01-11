use crate::client::MessagePassthru;
use crate::proto::Worker;
use crate::raw_message;
use std::sync::{Arc, RwLock};

pub trait WorkerSource {
    fn register_worker(&mut self, worker: Worker);
    fn get_worker(&self, name: &str) -> Option<&Worker>;
    fn unregister_worker(&mut self, name: &str);
}

pub struct WorkerServer<W: WorkerSource> {
    /// Own websocket end
    sender: ws::Sender,

    /// Source of worker data
    source: Arc<RwLock<W>>,

    /// Pass messages along the core connection
    corepass: MessagePassthru,
}

impl<W: WorkerSource> WorkerServer<W> {
    fn create(sender: ws::Sender, source: Arc<RwLock<W>>, corepass: MessagePassthru) -> Self {
        Self {
            sender,
            source,
            corepass,
        }
    }
}

impl<W: WorkerSource> ws::Handler for WorkerServer<W> {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        if req.protocols()?.contains(&&"armstrong/worker") {
            let mut res = ws::Response::from_request(req)?;
            res.set_protocol("armstrong/worker");
            Ok(res)
        } else {
            Err(ws::Error::new(ws::ErrorKind::Protocol, "wrong protocol"))
        }
    }

    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        info!("connection accepted for worker");
        // send greeting
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // handle server messages
        match msg {
            ws::Message::Text(rpc) => {
                //
            }
            ws::Message::Binary(raw) => {
                trace!("raw message received");
                if let Some((_header, _body)) = raw_message::parse(&raw) {
                    // TODO: parse header as RPC. Body is extra binary data. E.g. binary stream data.
                }
            }
        };
        Ok(())
    }

    fn on_shutdown(&mut self) {
        // info!() something out
    }
}