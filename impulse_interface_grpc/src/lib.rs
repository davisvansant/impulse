use tonic::{transport::Server, Request, Response, Status};

use impulse::external_server::{External, ExternalServer};
use impulse::{
    Empty, LaunchVmResponse, MicroVm, ShutdownVmResponse, SystemStatusResponse,
    SystemVersionResponse,
};

pub mod impulse {
    include!("../../proto/impulse.interface.v010.rs");
}

#[derive(Default)]
pub struct Impulse {}

#[tonic::async_trait]
impl External for Impulse {
    async fn system_status(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<SystemStatusResponse>, Status> {
        unimplemented!()
    }

    async fn system_version(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<SystemVersionResponse>, Status> {
        unimplemented!()
    }

    async fn launch_vm(
        &self,
        _request: Request<MicroVm>,
    ) -> Result<Response<LaunchVmResponse>, Status> {
        unimplemented!()
    }

    async fn shutdown_vm(
        &self,
        _request: Request<MicroVm>,
    ) -> Result<Response<ShutdownVmResponse>, Status> {
        unimplemented!()
    }
}
