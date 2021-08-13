use crate::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ConfigFile {
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
    async fn build() -> Result<ConfigFile, Box<dyn std::error::Error>> {
        let uuid = String::from("some_uuid");
        let boot_source = BootSource::build(&uuid).await?;
        let mut drives = Vec::with_capacity(3);
        let drive = Drive::build(false, true, &uuid).await?;

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
        let boot_args = String::from("some_boot_args");
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
        let drive = String::from("some_drive");
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

    #[tokio::test(flavor = "multi_thread")]
    async fn build() -> Result<(), Box<dyn std::error::Error>> {
        let test_config_file = ConfigFile::build().await?;
        assert_eq!(
            &test_config_file
                .boot_source
                .kernel_image_path
                .to_str()
                .unwrap(),
            &"/srv/impulse_actuator/some_uuid/some_kernel_image",
        );
        assert_eq!(
            &test_config_file.boot_source.boot_args.as_str(),
            &"some_boot_args",
        );
        assert_eq!(
            &test_config_file.boot_source.initrd_path.to_str().unwrap(),
            &"/srv/impulse_actuator/some_uuid/some_initrd",
        );

        for drive in &test_config_file.drives {
            assert_eq!(&drive.drive_id.as_str(), &"some_drive_id");
            assert!(!&drive.is_read_only);
            assert!(&drive.is_root_device);
            assert_eq!(
                &drive.path_on_host.to_str().unwrap(),
                &"/srv/impulse_actuator/some_uuid/some_drive",
            );
        }

        assert!(&test_config_file.machine_config.ht_enabled);
        assert_eq!(&test_config_file.machine_config.mem_size_mib, &1024);
        assert_eq!(&test_config_file.machine_config.vcpu_count, &2);

        Ok(())
    }
}
