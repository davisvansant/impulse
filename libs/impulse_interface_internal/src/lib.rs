use tonic::{Request, Response, Status};

use tokio::sync::broadcast::Sender;

use tokio_stream::wrappers::ReceiverStream;

use uuid::Uuid;

pub use internal_v010::interface_server::{Interface, InterfaceServer};
pub use internal_v010::{NodeId, SystemId, Task};

mod internal_v010 {
    include!("../../../proto/impulse.internal.v010.rs");
}

// #[derive(Default)]
pub struct Internal {
    pub system_id: Uuid,
    nodes: tokio::sync::Mutex<Vec<String>>,
    sender_clone: Sender<u8>,
}

impl Internal {
    pub async fn init(sender_clone: Sender<u8>) -> Result<Internal, Box<dyn std::error::Error>> {
        let system_id = Uuid::new_v4();
        let nodes = tokio::sync::Mutex::new(Vec::with_capacity(20));

        Ok(Internal {
            system_id,
            nodes,
            sender_clone,
        })
    }
}

#[tonic::async_trait]
impl Interface for Internal {
    async fn register(&self, request: Request<NodeId>) -> Result<Response<SystemId>, Status> {
        println!(
            ":: i m p u l s e _ i n t e r f a c e > New register request! | {}",
            request.get_ref().node_id,
        );

        let mut nodes = self.nodes.lock().await;
        let node = request.into_inner().node_id;

        nodes.push(node);

        println!(":: i m p u l s e _ i n t e r f a c e > Node Registered!");

        let system_id = SystemId {
            system_id: self.system_id.to_string(),
        };
        let response = Response::new(system_id);

        Ok(response)
    }

    type ControllerStream = ReceiverStream<Result<Task, Status>>;

    async fn controller(
        &self,
        request: tonic::Request<NodeId>,
    ) -> Result<tonic::Response<Self::ControllerStream>, tonic::Status> {
        println!("{:?}", request);

        let nodes = self.nodes.lock().await;

        if nodes.contains(&request.get_ref().node_id) {
            let (tx, rx) = tokio::sync::mpsc::channel(4);
            let mut receiver = self.sender_clone.subscribe();

            tokio::spawn(async move {
                while let Ok(msg) = receiver.recv().await {
                    println!("Message received - {:?}", msg);
                    match msg {
                        1 => {
                            println!("Start instance! {:?}", &msg);

                            let id = Uuid::new_v4().to_simple().to_string();
                            let task = Task { action: 1, id };

                            tx.send(Ok(task)).await.unwrap();
                        }
                        2 => {
                            println!("Shutdown instance! {:?}", &msg);

                            let id = Uuid::new_v4().to_simple().to_string();
                            let task = Task { action: 2, id };

                            tx.send(Ok(task)).await.unwrap();
                        }
                        _ => (),
                    }
                }
            });

            Ok(Response::new(ReceiverStream::new(rx)))
        } else {
            let message = String::from("Node was not found... please register first!");
            let status = Status::new(tonic::Code::NotFound, message);
            Err(status)
        }
    }

    async fn delist(&self, request: Request<NodeId>) -> Result<Response<SystemId>, Status> {
        let mut nodes = self.nodes.lock().await;
        let node_id = request.into_inner().node_id;

        match nodes.binary_search(&node_id) {
            Ok(node) => {
                let element = &nodes.get(node).unwrap();
                println!("removing node... {}", element);
                nodes.remove(node);
                println!("node removed...");

                let response = SystemId {
                    system_id: self.system_id.to_string(),
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
    use std::str::FromStr;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _test_rx) = tokio::sync::broadcast::channel(1);
        let test_internal = Internal::init(test_tx).await?;
        let test_nodes = test_internal.nodes.lock().await;
        assert_eq!(test_internal.system_id.get_version_num(), 4);
        assert_eq!(test_nodes.len(), 0);
        assert_eq!(test_nodes.capacity(), 20);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn register() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _test_rx) = tokio::sync::broadcast::channel(1);
        let test_internal = Internal::init(test_tx).await?;
        let test_nodes = test_internal.nodes.lock().await;
        assert_eq!(test_nodes.len(), 0);
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        let test_internal_register = test_internal.register(test_request).await?;
        let test_internal_register_uuid =
            Uuid::from_str(test_internal_register.get_ref().system_id.as_str()).unwrap();
        assert_eq!(test_internal_register_uuid.get_version_num(), 4);
        let test_nodes = test_internal.nodes.lock().await;
        assert_eq!(test_nodes.len(), 1);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn controller_response() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _test_rx) = tokio::sync::broadcast::channel(1);
        let test_tx_clone = test_tx.clone();
        let test_internal = Internal::init(test_tx_clone).await?;
        let mut test_nodes = test_internal.nodes.lock().await;
        test_nodes.push(String::from("test_uuid"));
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        let test_internal_controller = test_internal.controller(test_request).await?;
        test_tx.send(1).unwrap();
        drop(test_tx);
        drop(test_internal);
        let mut test_internal_controller_receiver = test_internal_controller.into_inner();
        while let Some(task) = test_internal_controller_receiver.as_mut().recv().await {
            match task.as_ref().unwrap().id.as_str() {
                "1" => assert_eq!(task.unwrap().action, 1),
                "2" => assert_eq!(task.unwrap().action, 2),
                _ => (),
            }
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn controller_status() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _test_rx) = tokio::sync::broadcast::channel(1);
        let test_tx_clone = test_tx.clone();
        let test_internal = Internal::init(test_tx_clone).await?;
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        test_tx.send(1).unwrap();
        let test_internal_controller = test_internal.controller(test_request).await;
        assert_eq!(
            test_internal_controller.as_ref().unwrap_err().code(),
            tonic::Code::NotFound,
        );
        assert_eq!(
            test_internal_controller.as_ref().unwrap_err().message(),
            "Node was not found... please register first!",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn delist_response() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _test_rx) = tokio::sync::broadcast::channel(1);
        let test_internal = Internal::init(test_tx).await?;
        let mut test_nodes = test_internal.nodes.lock().await;
        test_nodes.push(String::from("test_uuid"));
        assert_eq!(test_nodes.len(), 1);
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        let test_internal_delist = test_internal.delist(test_request).await?;
        let test_internal_delist_uuid =
            Uuid::from_str(test_internal_delist.get_ref().system_id.as_str()).unwrap();
        assert_eq!(test_internal_delist_uuid.get_version_num(), 4);
        let test_nodes = test_internal.nodes.lock().await;
        assert_eq!(test_nodes.len(), 0);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn delist_status() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _test_rx) = tokio::sync::broadcast::channel(1);
        let test_internal = Internal::init(test_tx).await?;
        let mut test_nodes = test_internal.nodes.lock().await;
        test_nodes.push(String::from("test_uuid"));
        assert_eq!(test_nodes.len(), 1);
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("not test_uuid"),
        });
        let test_internal_delist = test_internal.delist(test_request).await;
        assert_eq!(
            test_internal_delist.as_ref().unwrap_err().code(),
            tonic::Code::NotFound,
        );
        assert_eq!(
            test_internal_delist.as_ref().unwrap_err().message(),
            "Node not test_uuid was not found... please try again!",
        );
        let test_nodes = test_internal.nodes.lock().await;
        assert_eq!(test_nodes.len(), 1);
        Ok(())
    }
}
