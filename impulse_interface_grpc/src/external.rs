use crate::{Request, Response, Status};

pub use external_v010::interface_server::{Interface, InterfaceServer};
pub use external_v010::{
    Empty, LaunchVmResponse, MicroVm, ShutdownVmResponse, SystemStatusResponse,
    SystemVersionResponse,
};

mod external_v010 {
    include!("../../proto/impulse.external.v010.rs");
}

// #[derive(Default)]
pub struct External {
    status: String,
    version: String,
}

impl External {
    pub async fn init() -> Result<External, Box<dyn std::error::Error>> {
        let status = String::from("Running!");
        let version = String::from("v0.1.0");

        Ok(External { status, version })
    }
}

#[tonic::async_trait]
impl Interface for External {
    async fn system_status(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<SystemStatusResponse>, Status> {
        println!("{:?}", request);
        let status = SystemStatusResponse {
            status: self.status.to_owned(),
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
            version: self.version.to_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_external = External::init().await?;
        assert_eq!(test_external.status.as_str(), "Running!");
        assert_eq!(test_external.version.as_str(), "v0.1.0");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn system_status() -> Result<(), Box<dyn std::error::Error>> {
        let test_external = External::init().await?;
        let test_request = Request::new(Empty {});
        let test_response = Response::new(SystemStatusResponse {
            status: String::from("Running!"),
        });
        let test_external_system_status = test_external.system_status(test_request).await?;
        assert_eq!(
            test_external_system_status.into_inner().status.as_str(),
            test_response.into_inner().status.as_str(),
        );
        Ok(())
    }
}
