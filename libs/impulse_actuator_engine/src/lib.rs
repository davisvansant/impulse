use std::path::PathBuf;
use std::process::Stdio;

use tokio::fs;
use tokio::process::Command;

pub struct Engine {
    pub firecracker_binary: PathBuf,
    pub jailer_binary: PathBuf,
    pub config_base: PathBuf,
    pub socket_base: PathBuf,
    pub working_base: PathBuf,
    pub running_pids: Vec<u32>,
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

        let running_pids = Vec::with_capacity(20);

        Ok(Engine {
            firecracker_binary,
            jailer_binary,
            config_base,
            socket_base,
            working_base,
            running_pids,
            active: true,
        })
    }

    pub async fn launch_vm(&mut self, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut api_socket = PathBuf::from(self.socket_base.as_path());
        api_socket.push(uuid);
        api_socket.set_extension("socket");

        println!(
            ":: i m p u l s e _ a c t u a t o r > Launching new VM with socket | {:?}",
            &api_socket,
        );

        let mut config_file = PathBuf::from(self.config_base.as_path());
        config_file.push(uuid);
        config_file.set_extension("json");

        println!(
            ":: i m p u l s e _ a c t u a t o r > Launching new VM with config | {:?}",
            &config_file,
        );

        let mut working_base = PathBuf::from(self.working_base.as_path());
        working_base.push(uuid);
        working_base.set_extension("json");

        println!(
            ":: i m p u l s e _ a c t u a t o r > Launching new VM with base | {:?}",
            &working_base,
        );

        let stdin = Stdio::null();
        let stdout = Stdio::null();
        let stderr = Stdio::null();

        let unit_name = format!("--unit={}", uuid);
        let unit_slice = format!("--slice={}", uuid);

        let command = Command::new("/usr/bin/systemd-run")
            .stdin(stdin)
            .stdout(stdout)
            .stderr(stderr)
            .arg(&unit_name)
            .arg(&unit_slice)
            .arg(&working_base)
            .arg(&self.firecracker_binary)
            .arg("--api-sock")
            .arg(&api_socket)
            .arg("--config-file")
            .arg(&config_file)
            .status()
            .await?;

        println!("{:?}", &command);

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
        assert!(test_engine.running_pids.is_empty());
        assert!(test_engine.active);
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn launch_vm() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_engine = Engine::init().await?;
        let test_engine_boot = test_engine.launch_vm("some_test_uuid").await;
        assert!(test_engine_boot.is_err());
        assert!(test_engine.running_pids.is_empty());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn shutdown_vm() {
        let mut test_engine = Engine::init().await.unwrap();
        let test_engine_shutdown_vm = test_engine.shutdown_vm().await;
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
}
