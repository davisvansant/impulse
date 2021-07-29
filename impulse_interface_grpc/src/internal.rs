use crate::{Request, Response, Status};

use std::sync::Mutex;

use tokio_stream::wrappers::ReceiverStream;

pub use internal_v010::interface_server::{Interface, InterfaceServer};
pub use internal_v010::{NodeId, SystemId, Task};

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
    async fn register(&self, request: Request<NodeId>) -> Result<Response<SystemId>, Status> {
        let mut nodes = self.nodes.lock().unwrap();
        let node = request.into_inner().node_id;

        nodes.push(node);

        let system_id = SystemId {
            system_id: self.system_id.to_owned(),
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

        let nodes = self.nodes.lock().unwrap();

        if nodes.contains(&request.get_ref().node_id) {
            let (tx, rx) = tokio::sync::mpsc::channel(4);

            let task_one = Task {
                action: 1,
                id: String::from("1"),
            };
            let task_two = Task {
                action: 2,
                id: String::from("2"),
            };
            let task_three = Task {
                action: 0,
                id: String::from("3"),
            };
            let task_four = Task {
                action: 1,
                id: String::from("4"),
            };

            tokio::spawn(async move {
                println!("sending task one...");
                tx.send(Ok(task_one)).await.unwrap();

                println!("sending task two...");
                tx.send(Ok(task_two)).await.unwrap();

                println!("sending task three...");
                tx.send(Ok(task_three)).await.unwrap();

                println!("sending task four...");
                tx.send(Ok(task_four)).await.unwrap();
            });

            Ok(Response::new(ReceiverStream::new(rx)))
        } else {
            let message = String::from("Node was not found... please register first!");
            let status = Status::new(tonic::Code::NotFound, message);
            Err(status)
        }
    }

    async fn delist(&self, request: Request<NodeId>) -> Result<Response<SystemId>, Status> {
        let mut nodes = self.nodes.lock().unwrap();
        let node_id = request.into_inner().node_id;

        match nodes.binary_search(&node_id) {
            Ok(node) => {
                let element = &nodes.get(node).unwrap();
                println!("removing node... {}", element);
                nodes.remove(node);
                println!("node removed...");

                let response = SystemId {
                    system_id: self.system_id.to_owned(),
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
    async fn register() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 0);
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        let test_internal_register = test_internal.register(test_request).await?;
        assert_eq!(
            test_internal_register.get_ref().system_id.as_str(),
            "some_uuid",
        );
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 1);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn controller_response() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let mut test_nodes = test_internal.nodes.lock().unwrap();
        test_nodes.push(String::from("test_uuid"));
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        let test_internal_controller = test_internal.controller(test_request).await?;
        let mut test_internal_controller_receiver = test_internal_controller.into_inner();
        while let Some(task) = test_internal_controller_receiver.as_mut().recv().await {
            match task.as_ref().unwrap().id.as_str() {
                "1" => assert_eq!(task.unwrap().action, 1),
                "2" => assert_eq!(task.unwrap().action, 2),
                "3" => assert_eq!(task.unwrap().action, 0),
                "4" => assert_eq!(task.unwrap().action, 1),
                _ => (),
            }
        }
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn controller_status() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
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
        let test_internal = Internal::init().await?;
        let mut test_nodes = test_internal.nodes.lock().unwrap();
        test_nodes.push(String::from("test_uuid"));
        assert_eq!(test_nodes.len(), 1);
        drop(test_nodes);
        let test_request = Request::new(NodeId {
            node_id: String::from("test_uuid"),
        });
        let test_internal_delist = test_internal.delist(test_request).await?;
        assert_eq!(
            test_internal_delist.get_ref().system_id.as_str(),
            "some_uuid",
        );
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 0);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn delist_status() -> Result<(), Box<dyn std::error::Error>> {
        let test_internal = Internal::init().await?;
        let mut test_nodes = test_internal.nodes.lock().unwrap();
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
        let test_nodes = test_internal.nodes.lock().unwrap();
        assert_eq!(test_nodes.len(), 1);
        Ok(())
    }
}
