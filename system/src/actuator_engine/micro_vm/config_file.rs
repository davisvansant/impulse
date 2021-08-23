use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use serde_json::to_string_pretty;

use tokio::fs::create_dir_all;
use tokio::fs::write;

#[derive(Deserialize, Serialize)]
pub struct ConfigFile {
    #[serde(rename = "boot-source")]
    boot_source: BootSource,
    drives: Vec<Drive>,
    #[serde(rename = "machine-config")]
    machine_config: MachineConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    balloon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "network-interfaces")]
    network_interfaces: Option<Vec<NetworkInterfaces>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vsock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mmds-config")]
    mmds_config: Option<bool>,
}

impl ConfigFile {
    pub async fn build(uuid: &str) -> Result<ConfigFile, Box<dyn std::error::Error>> {
        let boot_source = BootSource::build(uuid).await?;
        let mut drives = Vec::with_capacity(3);
        let drive = Drive::build(false, true, uuid).await?;

        drives.push(drive);

        let machine_config = MachineConfig::build().await?;

        Ok(ConfigFile {
            boot_source,
            drives,
            machine_config,
            balloon: None,
            network_interfaces: None,
            vsock: None,
            logger: None,
            metrics: None,
            mmds_config: None,
        })
    }

    pub async fn write(&self, uuid: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut config_file = PathBuf::from("/var/lib/impulse_actuator/machine/");
        config_file.push(uuid);
        create_dir_all(&config_file).await?;
        config_file.push("config_file");
        config_file.set_extension("json");
        let contents = to_string_pretty(&self)?;
        write(&config_file, contents).await?;

        Ok(config_file)
    }
}

#[derive(Deserialize, Serialize)]
struct BootSource {
    kernel_image_path: PathBuf,
    boot_args: String,
    initrd_path: PathBuf,
}

impl BootSource {
    async fn build(uuid: &str) -> Result<BootSource, Box<dyn std::error::Error>> {
        let kernel_image = "some_kernel_image";
        let kernel_image_path =
            PathBuf::from(format!("/srv/impulse_actuator/{}/{}", uuid, kernel_image));
        let boot_args = String::from("console=ttyS0 reboot=k panic=1 pci=off");
        let initrd = "some_initrd";
        let initrd_path = PathBuf::from(format!("/srv/impulse_actuator/{}/{}", uuid, initrd));

        Ok(BootSource {
            kernel_image_path,
            boot_args,
            initrd_path,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct Drive {
    drive_id: String,
    is_read_only: bool,
    is_root_device: bool,
    path_on_host: PathBuf,
}

impl Drive {
    async fn build(
        is_read_only: bool,
        is_root_device: bool,
        uuid: &str,
    ) -> Result<Drive, Box<dyn std::error::Error>> {
        let drive_id = String::from("some_drive_id");
        let drive = String::from("some_root_fs");
        let path_on_host = PathBuf::from(format!("/srv/impulse_actuator/{}/{}", uuid, drive));

        Ok(Drive {
            drive_id,
            is_read_only,
            is_root_device,
            path_on_host,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct MachineConfig {
    ht_enabled: bool,
    mem_size_mib: u16,
    vcpu_count: u8,
}

impl MachineConfig {
    async fn build() -> Result<MachineConfig, Box<dyn std::error::Error>> {
        let mem_size_mib = 1024;
        let vcpu_count = 2;

        Ok(MachineConfig {
            ht_enabled: true,
            mem_size_mib,
            vcpu_count,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct NetworkInterfaces {
    host_dev_name: String,
    iface_id: String,
    guest_mac: String,
}

impl NetworkInterfaces {
    async fn build(mac_address: &str) -> Result<NetworkInterfaces, Box<dyn std::error::Error>> {
        let host_dev_name = String::from("tap0");
        let iface_id = String::from("eth0");
        let guest_mac = mac_address.to_string();

        Ok(NetworkInterfaces {
            host_dev_name,
            iface_id,
            guest_mac,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_UUID: uuid::Uuid = uuid::Uuid::nil();

    #[tokio::test(flavor = "multi_thread")]
    async fn build() -> Result<(), Box<dyn std::error::Error>> {
        let test_config_file =
            ConfigFile::build(TEST_UUID.to_simple().to_string().as_str()).await?;
        assert_eq!(
            test_config_file
                .boot_source
                .kernel_image_path
                .to_str()
                .unwrap(),
            "/srv/impulse_actuator/00000000000000000000000000000000/some_kernel_image",
        );
        assert_eq!(
            test_config_file.boot_source.boot_args.as_str(),
            "console=ttyS0 reboot=k panic=1 pci=off",
        );
        assert_eq!(
            test_config_file.boot_source.initrd_path.to_str().unwrap(),
            "/srv/impulse_actuator/00000000000000000000000000000000/some_initrd",
        );

        for drive in test_config_file.drives {
            assert_eq!(drive.drive_id.as_str(), "some_drive_id");
            assert!(!drive.is_read_only);
            assert!(drive.is_root_device);
            assert_eq!(
                drive.path_on_host.to_str().unwrap(),
                "/srv/impulse_actuator/00000000000000000000000000000000/some_root_fs",
            );
        }

        assert!(test_config_file.machine_config.ht_enabled);
        assert_eq!(test_config_file.machine_config.mem_size_mib, 1024);
        assert_eq!(test_config_file.machine_config.vcpu_count, 2);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn write() -> Result<(), Box<dyn std::error::Error>> {
        let test_config_file =
            ConfigFile::build(TEST_UUID.to_simple().to_string().as_str()).await?;
        test_config_file
            .write(TEST_UUID.to_simple().to_string().as_str())
            .await?;
        let test_config_file_metadata = tokio::fs::metadata(
            "/var/lib/impulse_actuator/machine/00000000000000000000000000000000/config_file.json",
        )
        .await?;
        assert!(test_config_file_metadata.is_file());
        let test_config_file_contents = tokio::fs::read(
            "/var/lib/impulse_actuator/machine/00000000000000000000000000000000/config_file.json",
        )
        .await?;
        let test_json: ConfigFile = serde_json::from_slice(&test_config_file_contents)?;
        assert_eq!(
            test_json.boot_source.kernel_image_path.to_str().unwrap(),
            "/srv/impulse_actuator/00000000000000000000000000000000/some_kernel_image",
        );
        assert_eq!(
            test_json.boot_source.boot_args.as_str(),
            "console=ttyS0 reboot=k panic=1 pci=off",
        );
        assert_eq!(
            test_json.boot_source.initrd_path.to_str().unwrap(),
            "/srv/impulse_actuator/00000000000000000000000000000000/some_initrd",
        );

        for drive in test_json.drives {
            assert_eq!(drive.drive_id.as_str(), "some_drive_id");
            assert!(!drive.is_read_only);
            assert!(drive.is_root_device);
            assert_eq!(
                drive.path_on_host.to_str().unwrap(),
                "/srv/impulse_actuator/00000000000000000000000000000000/some_root_fs",
            );
        }

        assert!(test_json.machine_config.ht_enabled);
        assert_eq!(test_json.machine_config.mem_size_mib, 1024);
        assert_eq!(test_json.machine_config.vcpu_count, 2);

        Ok(())
    }
}
