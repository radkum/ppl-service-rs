use std::{thread, time::Duration};

use windows_sys::Win32::{
    Storage::FileSystem::{FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, OPEN_EXISTING},
    System::Services::*,
};

use crate::{
    winapi,
    winapi::{error::WinapiError, handle_wrapper::SmartHandle, install_elam_cert},
};

pub(crate) fn create_normal_service(exe_file: &str, service_name: &str) -> Result<(), WinapiError> {
    let _sh_service = imp_create_service(exe_file, service_name)?;
    Ok(())
}

pub(crate) fn create_protected_service(
    exe_file: &str,
    service_name: &str,
) -> Result<(), WinapiError> {
    install_elam()?;

    let sm_service = imp_create_service(exe_file, service_name)?;

    set_service_protection(sm_service.clone(), true)
}

fn set_service_protection(sh_service: SmartHandle, protect: bool) -> Result<(), WinapiError> {
    let dwLaunchProtected = if protect {
        SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT
    } else {
        SERVICE_LAUNCH_PROTECTED_NONE
    };

    let info: SERVICE_LAUNCH_PROTECTED_INFO = SERVICE_LAUNCH_PROTECTED_INFO { dwLaunchProtected };

    winapi::change_service_config2(sh_service.get_raw(), SERVICE_CONFIG_LAUNCH_PROTECTED, &info)
}
fn imp_create_service(exe_file: &str, service_name: &str) -> Result<SmartHandle, WinapiError> {
    let manager_access = SC_MANAGER_ALL_ACCESS;
    let h_manager = winapi::open_sc_manager(manager_access)?;

    //Get full path to service exe with argv
    let service_path = std::path::Path::new(exe_file);
    if !service_path.exists() {
        //something wrong
        todo!()
    }
    let service_cmd = service_path.canonicalize()?;
    let service_cmd = service_cmd.to_str().unwrap_or_default();

    let h_service =
        winapi::create_service(h_manager.get_raw(), service_name, manager_access, service_cmd)?;

    println!("Success to create service: {}", service_name);
    Ok(h_service)
}

fn install_elam() -> Result<(), WinapiError> {
    let file_handle = winapi::create_file(
        "elam_rs.sys",
        FILE_READ_DATA,
        FILE_SHARE_READ,
        None,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        None,
    )?;
    install_elam_cert(file_handle.get_raw())?;

    log::info!("install_elam: Installed ELAM certificate");

    Ok(())
}

pub(crate) fn remove_protected_service(service_name: &str) -> Result<(), WinapiError> {
    install_elam()?;

    let manager_access = SC_MANAGER_ALL_ACCESS;
    let sh_manager = winapi::open_sc_manager(manager_access)?;

    let sh_service = winapi::open_service(sh_manager.get_raw(), service_name, SERVICE_ALL_ACCESS)?;

    set_service_protection(sh_service.clone(), false)?;

    let mut status_process_info = winapi::query_service_status_process_info(sh_service.get_raw())?;
    for _ in 0..10 {
        if status_process_info.dwCurrentState == SERVICE_STOPPED {
            break;
        }

        status_process_info = winapi::stop_service(sh_service.get_raw())?;
        thread::sleep(Duration::from_secs(1));
    }

    if status_process_info.dwCurrentState != SERVICE_STOPPED {
        //something wrong!!!
        panic!("service won't stop")
    }

    winapi::delete_service(sh_service.get_raw())?;
    println!("Success to remove service: {}", service_name);

    Ok(())
}

pub(crate) fn remove_service(service_name: &str) -> Result<(), WinapiError> {
    let manager_access = SC_MANAGER_ALL_ACCESS;
    let h_manager = winapi::open_sc_manager(manager_access)?;

    let h_service = winapi::open_service(h_manager.get_raw(), service_name, SERVICE_ALL_ACCESS)?;

    let mut status_process_info = winapi::query_service_status_process_info(h_service.get_raw())?;
    for _ in 0..10 {
        if status_process_info.dwCurrentState == SERVICE_STOPPED {
            break;
        }

        status_process_info = winapi::stop_service(h_service.get_raw())?;
        thread::sleep(Duration::from_secs(1));
    }

    if status_process_info.dwCurrentState != SERVICE_STOPPED {
        //something wrong!!!
        panic!("service won't stop")
    }

    winapi::delete_service(h_service.get_raw())?;
    println!("Success to remove service: {}", service_name);

    Ok(())
}
