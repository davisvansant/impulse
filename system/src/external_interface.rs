use tonic::{Request, Response, Status};

use tokio::sync::broadcast::Sender;

use uuid::Uuid;

use crate::impulse::external::v010::{MicroVm, SystemStatusResponse, SystemVersionResponse};
use crate::impulse::shared::v010::{Empty, MicroVmLaunch, MicroVmShutdown, Task};
use crate::IMPULSE_INTERFACE;

pub use crate::impulse::external::v010::interface_server::{Interface, InterfaceServer};

pub struct External {
    status: String,
    pub version: String,
    task_sender: Sender<Task>,
    launch_result_sender_clone: Sender<MicroVmLaunch>,
    shutdown_result_sender_clone: Sender<MicroVmShutdown>,
}

impl External {
    pub async fn init(
        task_sender: Sender<Task>,
        launch_result_sender_clone: Sender<MicroVmLaunch>,
        shutdown_result_sender_clone: Sender<MicroVmShutdown>,
    ) -> Result<External, Box<dyn std::error::Error>> {
        let status = String::from("Running!");
        let version = String::from("v0.1.0");

        Ok(External {
            status,
            version,
            task_sender,
            launch_result_sender_clone,
            shutdown_result_sender_clone,
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

    async fn launch_vm(&self, request: Request<Empty>) -> Result<Response<MicroVmLaunch>, Status> {
        println!(
            "{} Incoming launch request | {:?}",
            IMPULSE_INTERFACE,
            request.get_ref(),
        );
        println!(
            "{} Sending request to connected nodes | {:?}",
            IMPULSE_INTERFACE,
            &self.task_sender.receiver_count(),
        );

        let task = Task {
            action: 1,
            id: Uuid::new_v4().to_simple().to_string(),
        };

        if let Ok(msg) = &self.task_sender.send(task) {
            println!("{} Message sent | {:?}", IMPULSE_INTERFACE, &msg);
        }

        let mut receiver = self.launch_result_sender_clone.subscribe();

        if let Ok(message) = receiver.recv().await {
            let response = Response::new(message);

            Ok(response)
        } else {
            let message = String::from("Something went wrong!");
            let status = Status::new(tonic::Code::NotFound, message);
            Err(status)
        }
    }

    async fn shutdown_vm(
        &self,
        request: Request<MicroVm>,
    ) -> Result<Response<MicroVmShutdown>, Status> {
        let task = Task {
            action: 2,
            id: request.into_inner().name,
        };

        if let Ok(task) = &self.task_sender.send(task) {
            println!("Message sent - {:?}", task);
        }

        let mut receiver = self.shutdown_result_sender_clone.subscribe();

        if let Ok(message) = receiver.recv().await {
            let response = Response::new(message);

            Ok(response)
        } else {
            let message = String::from("Something went wrong!");
            let status = Status::new(tonic::Code::NotFound, message);
            Err(status)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let (test_response_sender, _) = tokio::sync::broadcast::channel(1);
        let test_response_sender_clone = test_response_sender.clone();
        let (test_shutdown_result_sender, _) = tokio::sync::broadcast::channel(1);
        let test_shutdown_result_sender_clone = test_shutdown_result_sender.clone();
        let test_external = External::init(
            test_tx,
            test_response_sender_clone,
            test_shutdown_result_sender_clone,
        )
        .await?;
        assert_eq!(test_external.status.as_str(), "Running!");
        assert_eq!(test_external.version.as_str(), "v0.1.0");
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn system_status() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let (test_response_sender, _) = tokio::sync::broadcast::channel(1);
        let test_response_sender_clone = test_response_sender.clone();
        let (test_shutdown_result_sender, _) = tokio::sync::broadcast::channel(1);
        let test_shutdown_result_sender_clone = test_shutdown_result_sender.clone();
        let test_external = External::init(
            test_tx,
            test_response_sender_clone,
            test_shutdown_result_sender_clone,
        )
        .await?;
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
        let (test_response_sender, _) = tokio::sync::broadcast::channel(1);
        let test_response_sender_clone = test_response_sender.clone();
        let (test_shutdown_result_sender, _) = tokio::sync::broadcast::channel(1);
        let test_shutdown_result_sender_clone = test_shutdown_result_sender.clone();
        let test_external = External::init(
            test_tx,
            test_response_sender_clone,
            test_shutdown_result_sender_clone,
        )
        .await?;
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
        let (test_response_sender, _test_response_rx) = tokio::sync::broadcast::channel(1);
        let test_response_sender_clone = test_response_sender.clone();
        drop(_test_response_rx);
        let (test_shutdown_result_sender, _) = tokio::sync::broadcast::channel(1);
        let test_shutdown_result_sender_clone = test_shutdown_result_sender.clone();
        let test_external = External::init(
            test_tx,
            test_response_sender_clone,
            test_shutdown_result_sender_clone,
        )
        .await?;
        let test_request = Request::new(Empty {});
        let test_result = tokio::spawn(async move {
            let test_external_launch_vm = test_external.launch_vm(test_request).await.unwrap();
            assert!(test_external_launch_vm.get_ref().launched);
            assert_eq!(
                test_external_launch_vm.get_ref().details.as_str(),
                "test_uuid"
            );
        });
        if test_response_sender.receiver_count() == 0 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        let test_instance_start = MicroVmLaunch {
            launched: true,
            details: String::from("test_uuid"),
        };
        test_response_sender
            .send(test_instance_start)
            .expect("could not send!");
        assert!(test_result.await.is_ok());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown_vm() -> Result<(), Box<dyn std::error::Error>> {
        let (test_tx, _rx) = tokio::sync::broadcast::channel(1);
        let (test_response_sender, _) = tokio::sync::broadcast::channel(1);
        let test_response_sender_clone = test_response_sender.clone();
        let (test_shutdown_result_sender, _) = tokio::sync::broadcast::channel(1);
        let test_shutdown_result_sender_clone = test_shutdown_result_sender.clone();
        let test_external = External::init(
            test_tx,
            test_response_sender_clone,
            test_shutdown_result_sender_clone,
        )
        .await?;
        let test_request = Request::new(MicroVm {
            name: String::from("tester"),
        });
        let test_result = tokio::spawn(async move {
            let test_external_shutdown_vm = test_external.shutdown_vm(test_request).await.unwrap();
            assert!(test_external_shutdown_vm.get_ref().shutdown);
            assert_eq!(
                test_external_shutdown_vm.get_ref().details.as_str(),
                "test_uuid",
            );
        });
        if test_shutdown_result_sender.receiver_count() == 0 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        let test_instance_shutdown = MicroVmShutdown {
            shutdown: true,
            details: String::from("test_uuid"),
        };
        test_shutdown_result_sender
            .send(test_instance_shutdown)
            .expect("could not send!");
        assert!(test_result.await.is_ok());
        Ok(())
    }
}
