use std::io::Write;

use prelude::{ELAM_DATA, ELAM_NAME};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, OPEN_EXISTING,
};

use crate::{winapi, winapi::error::WinapiError};

#[cfg(windows)]
pub(super) mod prelude {
    pub const ELAM_NAME: &str = "elam_rs.sys";

    #[cfg(target_arch = "x86_64")]
    pub const ELAM_DATA: &[u8] = include_bytes!("../../target/debug/elam_rs.sys");
    //pub const ELAM_DATA: &[u8] = include_bytes!(concat!(env!("CARGO_TARGET_DIR"), env!("PROFILE"), "/elam_rs.sys"));
}

pub(super) fn unpack_elam() -> Result<std::path::PathBuf, WinapiError> {
    let path = std::env::current_exe()?.with_file_name(ELAM_NAME);

    let mut decompressed = Vec::new();

    let data = {
        use std::io::Read;

        use flate2::read::GzDecoder as Decoder;

        let mut decoder = Decoder::new(ELAM_DATA);

        match decoder.read_to_end(&mut decompressed) {
            Ok(_) => decompressed.as_slice(),
            Err(_) => ELAM_DATA,
        }
    };

    //TODO: check hash
    // if path.exists() {
    //     let hash_path = path.as_path();
    //     let data_hash = Enricher::hash_data(data)?;
    //     let file_hash = hash_file!(hash_path)?;
    //
    //     if data_hash.sha256 == file_hash.sha256 {
    //         return Ok(path);
    //     }
    // }

    let mut file = std::fs::File::create(&path)?;
    file.write_all(data)?;

    Ok(path)
}

pub(super) fn install_elam(elam_path: &str) -> Result<(), WinapiError> {
    let file_handle = winapi::create_file(
        elam_path,
        FILE_READ_DATA,
        FILE_SHARE_READ,
        None,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        None,
    )?;
    winapi::install_elam_cert(file_handle.get_raw())?;

    log::info!("install_elam: Installed ELAM certificate");

    Ok(())
}
