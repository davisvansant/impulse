use crate::{Request, Response, Status};

use tokio::sync::broadcast::Sender;

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
    sender: Sender<u8>,
}

impl External {
    pub async fn init(sender: Sender<u8>) -> Result<External, Box<dyn std::error::Error>> {
        let status = String::from("Running!");
        let version = String::from("v0.1.0");

        Ok(External {
            status,
            version,
            sender,
        })
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
                println!("{:?}", &self.sender.receiver_count());
                if let Ok(msg) = &self.sender.send(1) {
                    println!("Message sent - {:?}", msg);
                }

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
                println!("{:?}", &self.sender.receiver_count());
                if let Ok(msg) = &self.sender.send(2) {
                    println!("Message sent - {:?}", msg);
                }

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
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        assert_eq!(test_external.status.as_str(), "Running!");
        assert_eq!(test_external.version.as_str(), "v0.1.0");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn system_status() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        let test_request = Request::new(Empty {});
        let test_external_system_status = test_external.system_status(test_request).await?;
        assert_eq!(
            test_external_system_status.get_ref().status.as_str(),
            "Running!",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn system_version() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        let test_request = Request::new(Empty {});
        let test_external_system_version = test_external.system_version(test_request).await?;
        assert_eq!(
            test_external_system_version.get_ref().version.as_str(),
            "v0.1.0",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn launch_vm_response() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        let test_request = Request::new(MicroVm {
            name: String::from("tester"),
        });
        let test_external_launch_vm = test_external.launch_vm(test_request).await?;
        assert!(test_external_launch_vm.get_ref().launched);
        assert_eq!(
            test_external_launch_vm.get_ref().details.as_str(),
            "vm has started!"
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn launch_vm_status() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        let test_request = Request::new(MicroVm {
            name: String::from("not tester"),
        });
        let test_external_launch_vm = test_external.launch_vm(test_request).await;
        assert_eq!(
            test_external_launch_vm.as_ref().unwrap_err().code(),
            tonic::Code::NotFound,
        );
        assert_eq!(
            test_external_launch_vm.as_ref().unwrap_err().message(),
            "The requested VM was not found... please try again!",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown_vm_response() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        let test_request = Request::new(MicroVm {
            name: String::from("tester"),
        });
        let test_external_shutdown_vm = test_external.shutdown_vm(test_request).await?;
        assert!(test_external_shutdown_vm.get_ref().shutdown);
        assert_eq!(
            test_external_shutdown_vm.get_ref().details.as_str(),
            "vm has been shutdown!",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown_vm_status() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let test_external = External::init(test_tx).await?;
        let test_request = Request::new(MicroVm {
            name: String::from("not tester"),
        });
        let test_external_shutdown_vm = test_external.shutdown_vm(test_request).await;
        assert_eq!(
            test_external_shutdown_vm.as_ref().unwrap_err().code(),
            tonic::Code::NotFound,
        );
        assert_eq!(
            test_external_shutdown_vm.as_ref().unwrap_err().message(),
            "The requested VM was not found... please try again!",
        );
        Ok(())
    }
}
