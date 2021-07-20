use std::path::PathBuf;

use tokio::fs;

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
        assert_eq!(
            test_actuator.jailer_binary.to_str().unwrap(),
            "/usr/bin/jailer",
        );
        assert_eq!(
            test_actuator.config_base_dir.to_str().unwrap(),
            "/var/lib/impulse/machine"
        );
        assert_eq!(
            test_actuator.socket_base.to_str().unwrap(),
            "/tmp/impulse/socket",
        );
        Ok(())
    }
}
