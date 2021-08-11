pub use internal_v010::interface_client::InterfaceClient;
pub use internal_v010::{NodeId, SystemId, Task};

use tonic::transport::Channel;
use tonic::{Request, Response, Status, Streaming};

use uuid::Uuid;

mod internal_v010 {
    include!("../../../proto/impulse.internal.v010.rs");
}

pub enum InterfaceClientRequest {
    Register,
    Controller,
    Delist,
}

impl InterfaceClientRequest {
    pub async fn build(&self, node_id: &Uuid) -> Request<NodeId> {
        Request::new(NodeId {
            node_id: node_id.to_string(),
        })
    }
}

pub struct Internal {
    client: InterfaceClient<Channel>,
    pub node_id: Uuid,
}

impl Internal {
    pub async fn init(endpoint: &'static str) -> Result<Internal, Box<dyn std::error::Error>> {
        let client = InterfaceClient::connect(endpoint).await?;
        let node_id = Uuid::new_v4();

        Ok(Internal { client, node_id })
    }

    pub async fn register(&mut self) -> Result<tonic::Response<SystemId>, tonic::Status> {
        let mut client = self.client.clone();
        let request = InterfaceClientRequest::Register.build(&self.node_id).await;
        let response = client.register(request).await?;

        Ok(response)
    }

    pub async fn controller(&mut self) -> Result<Response<Streaming<Task>>, Status> {
        let mut client = self.client.clone();
        let request = InterfaceClientRequest::Controller
            .build(&self.node_id)
            .await;
        let response = client.controller(request).await?;

        Ok(response)
    }

    pub async fn delist(&mut self) -> Result<Response<SystemId>, Status> {
        let mut client = self.client.clone();
        let request = InterfaceClientRequest::Delist.build(&self.node_id).await;
        let response = client.delist(request).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const TEST_ADDR: &str = "127.0.0.1:1284";
    const TEST_ENDPOINT: &str = "http://127.0.0.1:1284";
    const TEST_UUID: Uuid = Uuid::nil();

    #[tokio::test(flavor = "multi_thread")]
    async fn interface_client_request_register() -> Result<(), Box<dyn std::error::Error>> {
        let test_request = InterfaceClientRequest::Register.build(&TEST_UUID).await;
        assert_eq!(
            test_request.get_ref().node_id.as_str(),
            "00000000-0000-0000-0000-000000000000",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn interface_client_request_controller() -> Result<(), Box<dyn std::error::Error>> {
        let test_request = InterfaceClientRequest::Controller.build(&TEST_UUID).await;
        assert_eq!(
            test_request.get_ref().node_id.as_str(),
            "00000000-0000-0000-0000-000000000000",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn interface_client_request_delist() -> Result<(), Box<dyn std::error::Error>> {
        let test_request = InterfaceClientRequest::Delist.build(&TEST_UUID).await;
        assert_eq!(
            test_request.get_ref().node_id.as_str(),
            "00000000-0000-0000-0000-000000000000",
        );
        Ok(())
    }

    // #[tokio::test(flavor = "multi_thread")]
    // async fn init() -> Result<(), Box<dyn std::error::Error>> {
    //     tokio::spawn(async move {
    //         let (test_tx, _) = tokio::sync::broadcast::channel(1);
    //         let test_interface_endpoint = std::net::SocketAddr::from_str(TEST_ADDR).unwrap();
    //         let test_sender_clone = test_tx.clone();
    //         let test_interface_internal =
    //             impulse_interface_internal::Internal::init(test_sender_clone)
    //                 .await
    //                 .unwrap();
    //         tonic::transport::Server::builder()
    //             .add_service(impulse_interface_internal::InterfaceServer::new(
    //                 test_interface_internal,
    //             ))
    //             .serve(test_interface_endpoint)
    //             .await
    //             .unwrap();
    //     });
    //     let test_internal = Internal::init(TEST_ENDPOINT).await?;
    //     assert_eq!(test_internal.node_id.as_str(), "test_client_uuid");
    //     Ok(())
    // }
    //
    // #[tokio::test(flavor = "multi_thread")]
    // async fn register() -> Result<(), Box<dyn std::error::Error>> {
    //     tokio::spawn(async move {
    //         let (test_tx, _) = tokio::sync::broadcast::channel(1);
    //         let test_interface_endpoint = std::net::SocketAddr::from_str(TEST_ADDR).unwrap();
    //         let test_sender_clone = test_tx.clone();
    //         let test_interface_internal =
    //             impulse_interface_internal::Internal::init(test_sender_clone)
    //                 .await
    //                 .unwrap();
    //         tonic::transport::Server::builder()
    //             .add_service(impulse_interface_internal::InterfaceServer::new(
    //                 test_interface_internal,
    //             ))
    //             .serve(test_interface_endpoint)
    //             .await
    //             .unwrap();
    //     });
    //     let mut test_internal = Internal::init(TEST_ENDPOINT).await?;
    //     let test_reponse = test_internal.register().await?;
    //     assert_eq!(test_reponse.get_ref().system_id.as_str(), "some_uuid");
    //     Ok(())
    // }
    //
    // #[tokio::test(flavor = "multi_thread")]
    // async fn delist() -> Result<(), Box<dyn std::error::Error>> {
    //     tokio::spawn(async move {
    //         let (test_tx, _) = tokio::sync::broadcast::channel(1);
    //         let test_interface_endpoint = std::net::SocketAddr::from_str(TEST_ADDR).unwrap();
    //         let test_sender_clone = test_tx.clone();
    //         let test_interface_internal =
    //             impulse_interface_internal::Internal::init(test_sender_clone)
    //                 .await
    //                 .unwrap();
    //         tonic::transport::Server::builder()
    //             .add_service(impulse_interface_internal::InterfaceServer::new(
    //                 test_interface_internal,
    //             ))
    //             .serve(test_interface_endpoint)
    //             .await
    //             .unwrap();
    //     });
    //     let mut test_internal = Internal::init(TEST_ENDPOINT).await?;
    //     test_internal.register().await?;
    //     let test_reponse = test_internal.delist().await?;
    //     assert_eq!(test_reponse.get_ref().system_id.as_str(), "some_uuid");
    //     Ok(())
    // }
}
