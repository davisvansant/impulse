use std::path::Path;

use tokio::fs::{create_dir_all, metadata, remove_dir_all, remove_file};

use crate::PathBuf;

use config_file::ConfigFile;

mod config_file;

pub struct MicroVM {
    pub api_socket: PathBuf,
    pub config_file: ConfigFile,
    pub base: PathBuf,
    pub unit_name: String,
    pub unit_slice: String,
}

impl MicroVM {
    pub async fn init(
        uuid: &str,
        socket_base: &Path,
        working_base: &Path,
    ) -> Result<MicroVM, Box<dyn std::error::Error>> {
        let mut api_socket = socket_base.to_path_buf();
        api_socket.push(&uuid.to_string());
        api_socket.set_extension("socket");

        let config_file = ConfigFile::build(&uuid.to_string()).await?;

        let mut base = working_base.to_path_buf();
        base.push(&uuid.to_string());

        create_dir_all(&base).await?;

        let unit_name = format!("--unit={}", uuid);
        let unit_slice = format!("--slice={}", uuid);

        Ok(MicroVM {
            api_socket,
            config_file,
            base,
            unit_name,
            unit_slice,
        })
    }

    pub async fn cleanup_api_socket(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(metadata) = metadata(&self.api_socket).await {
            if metadata.is_file() {
                remove_file(&self.api_socket).await?;
            }
        }

        Ok(())
    }

    pub async fn cleanup_base(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(metadata) = metadata(&self.base).await {
            if metadata.is_dir() {
                remove_dir_all(&self.base).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MICROVM_UUID: crate::Uuid = crate::Uuid::nil();
    const TEST_SOCKET_BASE: &str = "/tmp/test_impulse_actuator/socket";
    const TEST_WORKING_BASE: &str = "/srv/test_impulse_actuator/";

    #[tokio::test(flavor = "multi_thread")]
    async fn init() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        let test_micro_vm_srv_metadata = metadata(&test_micro_vm.base).await?;
        assert!(test_micro_vm_srv_metadata.is_dir());
        assert_eq!(
            test_micro_vm.api_socket.to_str().unwrap(),
            "/tmp/test_impulse_actuator/socket/00000000000000000000000000000000.socket",
        );
        assert_eq!(
            test_micro_vm.base.to_str().unwrap(),
            "/srv/test_impulse_actuator/00000000000000000000000000000000",
        );
        assert_eq!(
            test_micro_vm.unit_name.as_str(),
            "--unit=00000000000000000000000000000000",
        );
        assert_eq!(
            test_micro_vm.unit_slice.as_str(),
            "--slice=00000000000000000000000000000000",
        );
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn cleanup_api_socket_ok() -> Result<(), Box<dyn std::error::Error>> {
        let test_socket = format!(
            "{}/{}",
            TEST_SOCKET_BASE,
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
        );
        tokio::fs::create_dir_all(test_socket).await?;

        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        tokio::fs::write(&test_micro_vm.api_socket, b"test socket").await?;
        let test_cleanup_api_socket = test_micro_vm.cleanup_api_socket().await;
        assert!(test_cleanup_api_socket.is_ok());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn cleanup_api_socket_error() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        let test_cleanup_api_socket = test_micro_vm.cleanup_api_socket().await;
        assert!(test_cleanup_api_socket.is_ok());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn cleanup_base_ok() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        tokio::fs::write(&test_micro_vm.base.join("test_file_1"), b"test base file 1").await?;
        tokio::fs::write(&test_micro_vm.base.join("test_file_2"), b"test base file 2").await?;
        tokio::fs::write(&test_micro_vm.base.join("test_file_3"), b"test base file 3").await?;
        let test_cleanup_base = test_micro_vm.cleanup_base().await;
        assert!(test_cleanup_base.is_ok());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn cleanup_base_error() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        remove_dir_all(&test_micro_vm.base).await?;
        let test_cleanup_base = test_micro_vm.cleanup_base().await;
        assert!(test_cleanup_base.is_ok());
        Ok(())
    }
}
