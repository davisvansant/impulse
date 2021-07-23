use crate::Streaming;
use crate::{Request, Response, Status};

use futures_core::Stream;

use std::pin::Pin;

pub use internal_v010::interface_server::{Interface, InterfaceServer};
pub use internal_v010::{
    ConnectionRequest, ConnectionResponse, ShutdownRequest, ShutdownResponse, Tasks,
};

mod internal_v010 {
    include!("../../proto/impulse.internal.v010.rs");
}

#[derive(Default)]
pub struct Internal {}

#[tonic::async_trait]
impl Interface for Internal {
    async fn connect(
        &self,
        _request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        unimplemented!()
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
