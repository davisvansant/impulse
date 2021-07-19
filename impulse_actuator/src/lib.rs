use std::path::PathBuf;

pub struct Actuator {
    pub firecracker_binary: PathBuf,
    pub jailer_binary: PathBuf,
}

impl Actuator {
    pub async fn init() -> Result<Actuator, Box<dyn std::error::Error>> {
        let firecracker_binary = PathBuf::from("/usr/bin/firecracker");
        let jailer_binary = PathBuf::from("/usr/bin/jailer");

        Ok(Actuator {
            firecracker_binary,
            jailer_binary,
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
        Ok(())
    }
}
