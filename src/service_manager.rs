mod elam;

use std::{thread, time::Duration};

use windows_sys::Win32::System::Services::*;

use crate::{
    service_manager::elam::{install_elam, unpack_elam},
    winapi,
    winapi::{error::WinapiError, handle_wrapper::SmartHandle},
};

pub(crate) fn create_normal_service(exe_file: &str, service_name: &str) -> Result<(), WinapiError> {
    let _sh_service = imp_create_service(exe_file, service_name)?;
    Ok(())
}

pub(crate) fn create_protected_service(
    exe_file: &str,
    service_name: &str,
) -> Result<(), WinapiError> {
    let elam_path = unpack_elam()?;
    install_elam(elam_path.to_str().unwrap_or_default())?;
    //install_elam("elam_rs.sys")?;

    let sm_service = imp_create_service(exe_file, service_name)?;

    set_service_protection(sm_service.clone())
}

fn set_service_protection(sh_service: SmartHandle) -> Result<(), WinapiError> {
    #[allow(non_snake_case)]
    let dwLaunchProtected = SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT;
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
        eprintln!("There is no file such as \"{}\"", exe_file);
    }
    let service_cmd = service_path.canonicalize()?;
    let service_cmd = service_cmd.to_str().unwrap_or_default();

    let h_service =
        winapi::create_service(h_manager.get_raw(), service_name, manager_access, service_cmd)?;

    println!("Success to create service: {}", service_name);
    Ok(h_service)
}

pub(crate) fn remove_protected_service(service_name: &str) -> Result<(), WinapiError> {
    let elam_path = unpack_elam()?;
    install_elam(elam_path.to_str().unwrap_or_default())?;
    //install_elam("elam_rs.sys")?;

    let manager_access = SC_MANAGER_ALL_ACCESS;
    let sh_manager = winapi::open_sc_manager(manager_access)?;

    let sh_service = winapi::open_service(sh_manager.get_raw(), service_name, SERVICE_ALL_ACCESS)?;

    let mut status_process_info = winapi::query_service_status_process_info(sh_service.get_raw())?;
    //log::debug!("Service status: {}", status_process_info);
    for _ in 0..10 {
        if status_process_info.dwCurrentState == SERVICE_RUNNING {
            break;
        }

        let res = winapi::start_service(sh_service.get_raw());
        if let Err(err) = res {
            println!("{err}");
        }
        status_process_info = winapi::query_service_status_process_info(sh_service.get_raw())?;
        thread::sleep(Duration::from_secs(1));
    }

    if status_process_info.dwCurrentState != SERVICE_RUNNING {
        //something wrong!!!
        println!("dwCurrentState: {}", status_process_info.dwCurrentState);
        println!("FAILURE: service won't start");
        return Ok(());
    }

    send_unprotect_control_code(sh_service.clone())?;

    for _ in 0..10 {
        status_process_info = winapi::query_service_status_process_info(sh_service.get_raw())?;
        if status_process_info.dwCurrentState == SERVICE_STOPPED {
            break;
        }

        let res = winapi::stop_service(sh_service.get_raw());
        match res {
            Ok(status) => status_process_info = status,
            Err(err) => println!("{err}"),
        }
        thread::sleep(Duration::from_secs(1));
    }

    if status_process_info.dwCurrentState != SERVICE_STOPPED {
        //something wrong!!!
        println!("dwCurrentState: {}", status_process_info.dwCurrentState);
        println!("FAILURE: service won't stop");
        return Ok(());
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

fn send_unprotect_control_code(handle: SmartHandle) -> Result<(), WinapiError> {
    const UNPROTECT_SELF: u32 = 0x00000080;
    winapi::send_service_control_code(handle.get_raw(), UNPROTECT_SELF)
}
