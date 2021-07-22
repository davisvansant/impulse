use tonic::{transport::Server, Request, Response, Status};

use external::interface_server::{Interface, InterfaceServer};
use external::{
    Empty, LaunchVmResponse, MicroVm, ShutdownVmResponse, SystemStatusResponse,
    SystemVersionResponse,
};

pub mod external {
    include!("../../proto/impulse.external.v010.rs");
}

#[derive(Default)]
pub struct External {}

#[tonic::async_trait]
impl Interface for External {
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
