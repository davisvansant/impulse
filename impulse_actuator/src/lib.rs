use std::path::PathBuf;
use std::process::Stdio;

use tokio::fs;
use tokio::process::Command;

pub struct Actuator {
    pub firecracker_binary: PathBuf,
    pub jailer_binary: PathBuf,
    pub config_base_dir: PathBuf,
    pub socket_base: PathBuf,
}

impl Actuator {
    pub async fn init() -> Result<Actuator, Box<dyn std::error::Error>> {
        let firecracker_binary = PathBuf::from("/usr/bin/firecracker");
        let jailer_binary = PathBuf::from("/usr/bin/jailer");

        let config_base_dir = PathBuf::from("/var/lib/impulse/machine");
        fs::create_dir_all(&config_base_dir).await?;

        let socket_base = PathBuf::from("/tmp/impulse/socket");
        fs::create_dir_all(&socket_base).await?;

        Ok(Actuator {
            firecracker_binary,
            jailer_binary,
            config_base_dir,
            socket_base,
        })
    }

    pub async fn boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = Stdio::null();
        let stdout = Stdio::null();
        let stderr = Stdio::null();
        let api_socket = self
            .socket_base
            .as_path()
            .join("socket_name_goes_here.socket");
        let config_file = self
            .config_base_dir
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
            test_actuator.config_base_dir.to_str().unwrap(),
            "/var/lib/impulse/machine"
        );
        let test_actuator_config_base_dir_metadata =
            fs::metadata(&test_actuator.config_base_dir).await?;
        assert!(test_actuator_config_base_dir_metadata.is_dir());
        assert_eq!(
            test_actuator.socket_base.to_str().unwrap(),
            "/tmp/impulse/socket",
        );
        let test_actuator_socket_base_metadata = fs::metadata(&test_actuator.socket_base).await?;
        assert!(test_actuator_socket_base_metadata.is_dir());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn boot() -> Result<(), Box<dyn std::error::Error>> {
        let test_actuator = Actuator::init().await?;
        let test_actuator_boot = test_actuator.boot().await;
        assert!(test_actuator_boot.is_err());
        Ok(())
    }
}
