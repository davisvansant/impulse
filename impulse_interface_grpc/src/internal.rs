use crate::Streaming;
use crate::{Request, Response, Status};

use futures_core::Stream;

use std::pin::Pin;

use std::sync::Mutex;

pub use internal_v010::interface_server::{Interface, InterfaceServer};
pub use internal_v010::{AttachRequest, AttachResponse, ShutdownRequest, ShutdownResponse, Tasks};

mod internal_v010 {
    include!("../../proto/impulse.internal.v010.rs");
}

// #[derive(Default)]
pub struct Internal {
    nodes: Mutex<Vec<String>>,
}

impl Internal {
    pub async fn init() -> Result<Internal, Box<dyn std::error::Error>> {
        let nodes = Mutex::new(Vec::with_capacity(20));

        Ok(Internal { nodes })
    }
}

#[tonic::async_trait]
impl Interface for Internal {
    async fn attach(
        &self,
        request: Request<AttachRequest>,
    ) -> Result<Response<AttachResponse>, Status> {
        let mut nodes = self.nodes.lock().unwrap();
        let node = request.into_inner().node_id;

        nodes.push(node);

        let system_id = AttachResponse {
            server_id: String::from("system_id"),
        };
        let response = Response::new(system_id);
        Ok(response)
    }

    type RunStream = Pin<Box<dyn Stream<Item = Result<Tasks, Status>> + Send + Sync>>;

    async fn run(
        &self,
        _request: Request<Streaming<Tasks>>,
    ) -> Result<Response<Self::RunStream>, Status> {
        unimplemented!()
    }

    async fn shutdown(
        &self,
        _request: Request<ShutdownRequest>,
    ) -> Result<Response<ShutdownResponse>, Status> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 0);
        assert_eq!(test_nodes.capacity(), 20);
        Ok(())
    }
}
