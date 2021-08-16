use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;

use tokio::fs;
use tokio::fs::remove_file;
use tokio::process::Command;

use uuid::adapter::Simple;
use uuid::Uuid;

mod config_file;
mod layer2;
mod layer3;
mod micro_vm;

use config_file::ConfigFile;
use layer2::Layer2;
use layer3::Layer3;
use micro_vm::MicroVM;

pub struct Engine {
    pub firecracker_binary: PathBuf,
    pub jailer_binary: PathBuf,
    pub config_base: PathBuf,
    pub socket_base: PathBuf,
    pub working_base: PathBuf,
    pub launched_vms: HashMap<Simple, MicroVM>,
    pub layer2: Layer2,
    pub layer3: Layer3,
    pub active: bool,
}

impl Engine {
    pub async fn init() -> Result<Engine, Box<dyn std::error::Error>> {
        let firecracker_binary = PathBuf::from("/usr/bin/firecracker");
        let jailer_binary = PathBuf::from("/usr/bin/jailer");

        let config_base = PathBuf::from("/var/lib/impulse_actuator/machine");
        fs::create_dir_all(&config_base).await?;

        let socket_base = PathBuf::from("/tmp/impulse_actuator/socket");
        fs::create_dir_all(&socket_base).await?;

        let working_base = PathBuf::from("/srv/impulse_actuator/");
        fs::create_dir_all(&working_base).await?;

        let launched_vms = HashMap::with_capacity(20);

        let layer2 = Layer2::init().await?;
        let layer3 = Layer3::init().await?;

        Ok(Engine {
            firecracker_binary,
            jailer_binary,
            config_base,
            socket_base,
            working_base,
            launched_vms,
            layer2,
            layer3,
            active: true,
        })
    }

    pub async fn launch_vm(&mut self, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let micro_vm = MicroVM::init(
            uuid,
            self.socket_base.as_path(),
            self.working_base.as_path(),
        )
        .await?;

        let config_file = ConfigFile::build(uuid).await?;
        let config_file_location = config_file.write(uuid).await?;

        println!(
            ":: i m p u l s e _ a c t u a t o r > Launching new VM | {:?}",
            uuid,
        );

        let stdin = Stdio::null();
        let stdout = Stdio::null();
        let stderr = Stdio::null();

        let command = Command::new("/usr/bin/systemd-run")
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .arg(&micro_vm.unit_name)
            .arg(&micro_vm.unit_slice)
            .arg(&self.firecracker_binary)
            .arg("--api-sock")
            .arg(&micro_vm.api_socket)
            .arg("--config-file")
            .arg(&config_file_location)
            .status()
            .await?;

        println!("{:?}", &command);

        if command.success() {
            let uuid = Self::parse_uuid(uuid).await?;
            if let Some(micro_vm) = self.launched_vms.insert(uuid, micro_vm) {
                println!(
                    ":: i m p u l s e _ a c t u a t o r > Launched new VM with socket | {:?}",
                    &micro_vm.api_socket,
                );
                println!(
                    ":: i m p u l s e _ a c t u a t o r > Launched new VM with config | {:?}",
                    &config_file_location,
                );
                println!(
                    ":: i m p u l s e _ a c t u a t o r > Launched new VM with base | {:?}",
                    &micro_vm.base,
                );
            }
        }

        Ok(())
    }

    pub async fn shutdown_vm(&mut self, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut api_socket = PathBuf::from(self.socket_base.as_path());
        api_socket.push(uuid);
        api_socket.set_extension("socket");

        let unit_slice = format!("{}.slice", uuid);

        println!(
            ":: i m p u l s e _ a c t u a t o r > Shutting down VM | {:?}",
            uuid,
        );

        let command = Command::new("/usr/bin/systemctl")
            .arg("stop")
            .arg(&unit_slice)
            .status()
            .await?;

        println!("{:?}", &command);

        if command.success() {
            let uuid = Self::parse_uuid(uuid).await?;
            if let Some(micro_vm) = self.launched_vms.remove(&uuid) {
                println!(
                    ":: i m p u l s e _ a c t u a t o r > Removing socket | {:?}",
                    &micro_vm.api_socket,
                );
                println!(
                    ":: i m p u l s e _ a c t u a t o r > Removing base | {:?}",
                    &micro_vm.base,
                );
                println!(":: i m p u l s e _ a c t u a t o r > Shutdown! |");
            }
        }

        println!(
            ":: i m p u l s e _ a c t u a t o r > Removing socket | {:?}",
            &api_socket,
        );

        remove_file(&api_socket).await?;

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.active {
            self.active = false;
        }

        Ok(())
    }

    async fn parse_uuid(uuid: &str) -> Result<Simple, Box<dyn std::error::Error>> {
        let parsed_uuid = Uuid::parse_str(uuid)?;

        Ok(parsed_uuid.to_simple())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LAUNCH_VM_UUID: uuid::Uuid = uuid::Uuid::nil();

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_engine = Engine::init().await?;
        assert_eq!(
            test_engine.firecracker_binary.to_str().unwrap(),
            "/usr/bin/firecracker",
        );
        let test_engine_fircracker_binary_metadata =
            fs::metadata(&test_engine.firecracker_binary).await;
        assert!(test_engine_fircracker_binary_metadata.is_err());
        assert_eq!(
            test_engine.jailer_binary.to_str().unwrap(),
            "/usr/bin/jailer",
        );
        let test_engine_jailer_binary_metadata = fs::metadata(&test_engine.jailer_binary).await;
        assert!(test_engine_jailer_binary_metadata.is_err());
        assert_eq!(
            test_engine.config_base.to_str().unwrap(),
            "/var/lib/impulse_actuator/machine"
        );
        let test_engine_config_base_metadata = fs::metadata(&test_engine.config_base).await?;
        assert!(test_engine_config_base_metadata.is_dir());
        assert_eq!(
            test_engine.socket_base.to_str().unwrap(),
            "/tmp/impulse_actuator/socket",
        );
        let test_engine_socket_base_metadata = fs::metadata(&test_engine.socket_base).await?;
        assert!(test_engine_socket_base_metadata.is_dir());
        let test_engine_working_base_metadata = fs::metadata(&test_engine.working_base).await?;
        assert!(test_engine_working_base_metadata.is_dir());
        assert_eq!(
            test_engine.working_base.to_str().unwrap(),
            "/srv/impulse_actuator/",
        );
        assert!(test_engine.launched_vms.is_empty());
        assert!(test_engine.active);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn launch_vm() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_engine = Engine::init().await?;
        let test_engine_boot = test_engine
            .launch_vm(TEST_LAUNCH_VM_UUID.to_simple().to_string().as_str())
            .await;
        assert!(test_engine_boot.is_err());
        assert!(test_engine.launched_vms.is_empty());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown_vm() {
        let mut test_engine = Engine::init().await.unwrap();
        let test_engine_shutdown_vm = test_engine
            .shutdown_vm(TEST_LAUNCH_VM_UUID.to_simple().to_string().as_str())
            .await;
        assert!(test_engine_shutdown_vm.is_err());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn shutdown() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_engine = Engine::init().await?;
        assert!(test_engine.active);
        test_engine.shutdown().await?;
        assert!(!test_engine.active);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parse_uuid() -> Result<(), Box<dyn std::error::Error>> {
        let test_parse_uuid =
            Engine::parse_uuid(TEST_LAUNCH_VM_UUID.to_simple().to_string().as_str()).await?;
        assert_eq!(
            test_parse_uuid,
            Uuid::parse_str("00000000000000000000000000000000")?.to_simple(),
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parse_uuid_error() -> Result<(), Box<dyn std::error::Error>> {
        let test_uuid = "0000000";
        let test_parse_uuid = Engine::parse_uuid(test_uuid).await;
        assert!(test_parse_uuid.is_err());
        Ok(())
    }
}
