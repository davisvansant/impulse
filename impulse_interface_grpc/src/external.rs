use crate::{Request, Response, Status};

pub use external_v010::interface_server::{Interface, InterfaceServer};
pub use external_v010::{
    Empty, LaunchVmResponse, MicroVm, ShutdownVmResponse, SystemStatusResponse,
    SystemVersionResponse,
};

mod external_v010 {
    include!("../../proto/impulse.external.v010.rs");
}

#[derive(Default)]
pub struct External {}

#[tonic::async_trait]
impl Interface for External {
    async fn system_status(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<SystemStatusResponse>, Status> {
        println!("{:?}", request);
        let status = SystemStatusResponse {
            status: String::from("its running!"),
        };
        let response = Response::new(status);
        Ok(response)
    }

    async fn system_version(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<SystemVersionResponse>, Status> {
        println!("{:?}", request);
        let version = SystemVersionResponse {
            version: String::from("v0.1.0"),
        };
        let response = Response::new(version);
        Ok(response)
    }

    async fn launch_vm(
        &self,
        request: Request<MicroVm>,
    ) -> Result<Response<LaunchVmResponse>, Status> {
        match request.into_inner().name.as_str() {
            "tester" => {
                let launch_vm = LaunchVmResponse {
                    launched: true,
                    details: String::from("vm has started!"),
                };
                let response = Response::new(launch_vm);
                Ok(response)
            }
            _ => {
                let message = String::from("The requested VM was not found... please try again!");
                let status = Status::new(tonic::Code::NotFound, message);
                Err(status)
            }
        }
    }

    async fn shutdown_vm(
        &self,
        request: Request<MicroVm>,
    ) -> Result<Response<ShutdownVmResponse>, Status> {
        match request.into_inner().name.as_str() {
            "tester" => {
                let shutdown_vm = ShutdownVmResponse {
                    shutdown: true,
                    details: String::from("vm has been shutdown!"),
                };
                let response = Response::new(shutdown_vm);
                Ok(response)
            }
            _ => {
                let message = String::from("The requested VM was not found... please try again!");
                let status = Status::new(tonic::Code::NotFound, message);
                Err(status)
            }
        }
    }
}
