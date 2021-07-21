use std::path::PathBuf;
use std::process::Stdio;

use tokio::fs;
use tokio::process::Command;

pub struct Actuator {
    pub firecracker_binary: PathBuf,
    pub jailer_binary: PathBuf,
    pub config_base: PathBuf,
    pub socket_base: PathBuf,
    pub running_pids: Vec<u32>,
    pub active: bool,
}

impl Actuator {
    pub async fn init() -> Result<Actuator, Box<dyn std::error::Error>> {
        let firecracker_binary = PathBuf::from("/usr/bin/firecracker");
        let jailer_binary = PathBuf::from("/usr/bin/jailer");

        let config_base = PathBuf::from("/var/lib/impulse/machine");
        fs::create_dir_all(&config_base).await?;

        let socket_base = PathBuf::from("/tmp/impulse/socket");
        fs::create_dir_all(&socket_base).await?;

        let running_pids = Vec::with_capacity(20);

        Ok(Actuator {
            firecracker_binary,
            jailer_binary,
            config_base,
            socket_base,
            running_pids,
            active: true,
        })
    }

    pub async fn launch_vm(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = Stdio::null();
        let stdout = Stdio::null();
        let stderr = Stdio::null();
        let api_socket = self
            .socket_base
            .as_path()
            .join("socket_name_goes_here.socket");
        let config_file = self
            .config_base
            .as_path()
            .join("config_file_name_goes_here.json");
        let command = Command::new(&self.firecracker_binary)
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .arg("--api-sock")
            .arg(api_socket)
            .arg("--config-file")
            .arg(config_file)
            .spawn()?;

        if let Some(id) = command.id() {
            self.running_pids.push(id)
        }

        Ok(())
    }

    pub async fn shutdown_vm(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.active {
            self.active = false;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_actuator = Actuator::init().await?;
        assert_eq!(
            test_actuator.firecracker_binary.to_str().unwrap(),
            "/usr/bin/firecracker",
        );
        let test_actuator_fircracker_binary_metadata =
            fs::metadata(&test_actuator.firecracker_binary).await;
        assert!(test_actuator_fircracker_binary_metadata.is_err());
        assert_eq!(
            test_actuator.jailer_binary.to_str().unwrap(),
            "/usr/bin/jailer",
        );
        let test_actuator_jailer_binary_metadata = fs::metadata(&test_actuator.jailer_binary).await;
        assert!(test_actuator_jailer_binary_metadata.is_err());
        assert_eq!(
            test_actuator.config_base.to_str().unwrap(),
            "/var/lib/impulse/machine"
        );
        let test_actuator_config_base_metadata = fs::metadata(&test_actuator.config_base).await?;
        assert!(test_actuator_config_base_metadata.is_dir());
        assert_eq!(
            test_actuator.socket_base.to_str().unwrap(),
            "/tmp/impulse/socket",
        );
        let test_actuator_socket_base_metadata = fs::metadata(&test_actuator.socket_base).await?;
        assert!(test_actuator_socket_base_metadata.is_dir());
        assert!(test_actuator.running_pids.is_empty());
        assert!(test_actuator.active);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn launch_vm() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_actuator = Actuator::init().await?;
        let test_actuator_boot = test_actuator.launch_vm().await;
        assert!(test_actuator_boot.is_err());
        assert!(test_actuator.running_pids.is_empty());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn shutdown_vm() {
        let mut test_actuator = Actuator::init().await.unwrap();
        let test_actuator_shutdown_vm = test_actuator.shutdown_vm().await;
        assert!(test_actuator_shutdown_vm.is_err());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_actuator = Actuator::init().await?;
        assert!(test_actuator.active);
        test_actuator.shutdown().await?;
        assert!(!test_actuator.active);
        Ok(())
    }
}
