use crate::Streaming;
use crate::{Request, Response, Status};

use futures_core::Stream;

use std::pin::Pin;

use std::sync::Mutex;

use tokio_stream::wrappers::ReceiverStream;

pub use internal_v010::interface_server::{Interface, InterfaceServer};
pub use internal_v010::{AttachRequest, AttachResponse, ShutdownRequest, ShutdownResponse, Tasks};

mod internal_v010 {
    include!("../../proto/impulse.internal.v010.rs");
}

// #[derive(Default)]
pub struct Internal {
    system_id: String,
    nodes: Mutex<Vec<String>>,
}

impl Internal {
    pub async fn init() -> Result<Internal, Box<dyn std::error::Error>> {
        let system_id = String::from("some_uuid");
        let nodes = Mutex::new(Vec::with_capacity(20));

        Ok(Internal { system_id, nodes })
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
            server_id: self.system_id.to_owned(),
        };
        let response = Response::new(system_id);
        Ok(response)
    }

    type RunStream = ReceiverStream<Result<Tasks, Status>>;

    async fn run(
        &self,
        request: Request<Streaming<Tasks>>,
    ) -> Result<Response<Self::RunStream>, Status> {
        println!("{:?}", request);

        let (tx, rx) = tokio::sync::mpsc::channel(4);

        match request.into_inner().message().await? {
            Some(task) => {
                println!("received... {:?}", task);
                tx.send(Ok(task)).await.unwrap();
            }
            None => println!("no more"),
        };

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn shutdown(
        &self,
        request: Request<ShutdownRequest>,
    ) -> Result<Response<ShutdownResponse>, Status> {
        let mut nodes = self.nodes.lock().unwrap();
        let node_id = request.into_inner().node_id;

        match nodes.binary_search(&node_id) {
            Ok(node) => {
                let element = &nodes.get(node).unwrap();
                println!("removing node... {}", element);
                nodes.remove(node);
                println!("node removed...");

                let response = ShutdownResponse {
                    system_id: String::from("node removed!"),
                };

                Ok(Response::new(response))
            }
            Err(error) => {
                println!("{}", error);
                let message = format!("Node {} was not found... please try again!", &node_id);
                let status = Status::new(tonic::Code::NotFound, message);
                Err(status)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_internal.system_id.as_str(), "some_uuid");
        assert_eq!(test_nodes.len(), 0);
        assert_eq!(test_nodes.capacity(), 20);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn attach() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 0);
        drop(test_nodes);
        let test_request = Request::new(AttachRequest {
            node_id: String::from("test_uuid"),
        });
        let test_internal_attach = test_internal.attach(test_request).await?;
        assert_eq!(
            test_internal_attach.get_ref().server_id.as_str(),
            "some_uuid",
        );
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 1);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown_response() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let mut test_nodes = test_internal.nodes.lock().unwrap();
        test_nodes.push(String::from("test_uuid"));
        assert_eq!(test_nodes.len(), 1);
        drop(test_nodes);
        let test_request = Request::new(ShutdownRequest {
            node_id: String::from("test_uuid"),
        });
        let test_internal_shutdown = test_internal.shutdown(test_request).await?;
        assert_eq!(
            test_internal_shutdown.get_ref().system_id.as_str(),
            "node removed!",
        );
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 0);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown_status() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let mut test_nodes = test_internal.nodes.lock().unwrap();
        test_nodes.push(String::from("test_uuid"));
        assert_eq!(test_nodes.len(), 1);
        drop(test_nodes);
        let test_request = Request::new(ShutdownRequest {
            node_id: String::from("not test_uuid"),
        });
        let test_internal_shutdown = test_internal.shutdown(test_request).await;
        assert_eq!(
            test_internal_shutdown.as_ref().unwrap_err().code(),
            tonic::Code::NotFound,
        );
        assert_eq!(
            test_internal_shutdown.as_ref().unwrap_err().message(),
            "Node not test_uuid was not found... please try again!",
        );
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 1);
        Ok(())
    }
}
