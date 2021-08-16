use crate::{ConfigFile, PathBuf};

use std::path::Path;

use uuid::adapter::Simple;
use uuid::Uuid;

pub struct MicroVM {
    pub uuid: Simple,
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
        let uuid = Uuid::parse_str(uuid)
            .expect("Could not parse UUID!")
            .to_simple();

        let mut api_socket = socket_base.to_path_buf();
        api_socket.push(&uuid.to_string());
        api_socket.set_extension("socket");

        let config_file = ConfigFile::build(&uuid.to_string()).await?;

        let mut base = working_base.to_path_buf();
        base.push(&uuid.to_string());

        let unit_name = format!("--unit={}", uuid);
        let unit_slice = format!("--slice={}", uuid);

        Ok(MicroVM {
            uuid,
            api_socket,
            config_file,
            base,
            unit_name,
            unit_slice,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MICROVM_UUID: Uuid = Uuid::nil();
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
        assert_eq!(
            test_micro_vm.uuid.to_string().as_str(),
            "00000000000000000000000000000000",
        );
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
}