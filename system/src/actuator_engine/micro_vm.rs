use std::path::Path;

use tokio::fs::{copy, create_dir_all, metadata, remove_dir_all, remove_file};

use std::path::PathBuf;

use config_file::ConfigFile;

mod config_file;

pub struct MicroVM {
    pub api_socket: PathBuf,
    pub config_file: ConfigFile,
    pub config_path: PathBuf,
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
        let config_path = config_file.write(uuid).await?;

        let mut base = working_base.to_path_buf();
        base.push(&uuid.to_string());

        create_dir_all(&base).await?;

        let unit_name = format!("--unit={}", uuid);
        let unit_slice = format!("--slice={}", uuid);

        Ok(MicroVM {
            api_socket,
            config_file,
            config_path,
            base,
            unit_name,
            unit_slice,
        })
    }

    pub async fn ready_boot(&self, images: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let kernel_image_name = "some_kernel_image";
        let initrd_name = "some_initrd";
        let root_fs_name = "some_root_fs";

        let base_kernel_image = images.join(&kernel_image_name);
        let base_initrd = images.join(&initrd_name);
        let base_root_fs = images.join(&root_fs_name);

        let running_kernel_image = self.base.as_path().join(&kernel_image_name);
        let running_initrd = self.base.as_path().join(&initrd_name);
        let running_root_fs = self.base.as_path().join(&root_fs_name);

        copy(base_kernel_image, running_kernel_image).await?;
        copy(base_initrd, running_initrd).await?;
        copy(base_root_fs, running_root_fs).await?;

        Ok(())
    }

    pub async fn cleanup_api_socket(&self) -> Result<(), Box<dyn std::error::Error>> {
        if metadata(&self.api_socket).await.is_ok() {
            remove_file(&self.api_socket).await?;
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

    pub async fn cleanup_config_path(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = &self.config_path.parent() {
            if let Ok(metadata) = metadata(parent).await {
                if metadata.is_dir() {
                    remove_dir_all(parent).await?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MICROVM_UUID: uuid::Uuid = uuid::Uuid::nil();
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
        assert!(metadata(&test_micro_vm.config_path)
            .await
            .unwrap()
            .is_file());
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
    async fn ready_boot() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        let test_images_base = Path::new("/var/lib/test_impulse_actuator/images");
        create_dir_all(test_images_base).await?;
        tokio::fs::write(
            test_images_base.join("some_kernel_image"),
            b"test kernel image",
        )
        .await?;
        tokio::fs::write(test_images_base.join("some_initrd"), b"test initrd").await?;
        tokio::fs::write(test_images_base.join("some_root_fs"), b"test root fs").await?;
        let test_ready_boot = test_micro_vm.ready_boot(test_images_base).await;
        assert!(test_ready_boot.is_ok());
        let test_kernel_image_md =
            metadata(&test_micro_vm.base.as_path().join("some_kernel_image")).await?;
        assert!(test_kernel_image_md.is_file());
        let test_initrd_md = metadata(&test_micro_vm.base.as_path().join("some_initrd")).await?;
        assert!(test_initrd_md.is_file());
        let test_root_fs_md = metadata(&test_micro_vm.base.as_path().join("some_root_fs")).await?;
        assert!(test_root_fs_md.is_file());
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
        assert!(metadata(&test_micro_vm.api_socket).await.is_ok());
        let test_cleanup_api_socket = test_micro_vm.cleanup_api_socket().await;
        assert!(metadata(&test_micro_vm.api_socket).await.is_err());
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
        assert!(metadata(&test_micro_vm.base).await.is_ok());
        let test_cleanup_base = test_micro_vm.cleanup_base().await;
        assert!(test_cleanup_base.is_ok());
        assert!(metadata(&test_micro_vm.base).await.is_err());
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
        assert!(metadata(&test_micro_vm.base).await.is_ok());
        remove_dir_all(&test_micro_vm.base).await?;
        let test_cleanup_base = test_micro_vm.cleanup_base().await;
        assert!(test_cleanup_base.is_ok());
        assert!(metadata(&test_micro_vm.base).await.is_err());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn cleanup_config_path_ok() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        assert!(metadata(&test_micro_vm.config_path).await.is_ok());
        let test_cleanup_config_path = test_micro_vm.cleanup_config_path().await;
        assert!(test_cleanup_config_path.is_ok());
        assert!(metadata(&test_micro_vm.config_path).await.is_err());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn cleanup_config_path_error() -> Result<(), Box<dyn std::error::Error>> {
        let test_micro_vm = MicroVM::init(
            TEST_MICROVM_UUID.to_simple().to_string().as_str(),
            Path::new(TEST_SOCKET_BASE),
            Path::new(TEST_WORKING_BASE),
        )
        .await?;
        assert!(metadata(&test_micro_vm.config_path).await.is_ok());
        remove_dir_all(&test_micro_vm.config_path.parent().unwrap()).await?;
        assert!(metadata(&test_micro_vm.config_path).await.is_err());
        let test_cleanup_config_path = test_micro_vm.cleanup_config_path().await;
        assert!(test_cleanup_config_path.is_ok());
        assert!(metadata(&test_micro_vm.config_path).await.is_err());
        Ok(())
    }
}
