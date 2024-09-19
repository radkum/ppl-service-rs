pub(crate) mod error;
pub(crate) mod error_msg;
pub(crate) mod handle_wrapper;

use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

use windows::core::imp::SECURITY_ATTRIBUTES;
use windows_sys::Win32::{
    Foundation::{FALSE, HANDLE},
    Storage::FileSystem::CreateFileA,
    System::{Antimalware::InstallELAMCertificateInfo, Services::*},
};

use crate::winapi::{
    error::{WinapiCallError, WinapiError},
    handle_wrapper::SmartHandle,
};

pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; // {0xffffffffffffffff as *mut core::ffi::c_void}

pub(crate) fn create_file(
    name: &str,
    desired_access: u32,
    share_mode: u32,
    _security_attr: Option<SECURITY_ATTRIBUTES>,
    creation_disp: u32,
    flags_and_attrs: u32,
    _template_file: Option<String>,
) -> Result<SmartHandle, WinapiError> {
    let driver_name = CString::new(name)?;
    let file_handle = unsafe {
        CreateFileA(
            driver_name.as_ptr() as *const u8,
            desired_access,
            share_mode,
            null_mut(),
            creation_disp,
            flags_and_attrs,
            null_mut(),
        )
    };

    if file_handle == INVALID_HANDLE_VALUE {
        Err(WinapiCallError::new("CreateFileA").into())
    } else {
        Ok(SmartHandle::from(file_handle))
    }
}

pub(crate) fn install_elam_cert(handle: HANDLE) -> Result<(), WinapiError> {
    let status = unsafe { InstallELAMCertificateInfo(handle) };

    if status == FALSE {
        Err(WinapiCallError::new("InstallELAMCertificateInfo").into())
    } else {
        Ok(())
    }
}

pub(crate) fn open_sc_manager(access: u32) -> Result<SmartHandle, WinapiError> {
    let h_manager = unsafe { OpenSCManagerA(null_mut(), null_mut(), access) };
    if h_manager.is_null() {
        Err(WinapiCallError::new("OpenSCManagerA").into())
    } else {
        Ok(h_manager.into())
    }
}

pub(crate) fn open_service(
    h_manager: HANDLE,
    service_name: &str,
    access: u32,
) -> Result<SmartHandle, WinapiError> {
    let service_name = CString::new(service_name)?;
    let h_service = unsafe { OpenServiceA(h_manager, service_name.as_ptr() as *const u8, access) };

    if h_service.is_null() {
        Err(WinapiCallError::new("OpenServiceA").into())
    } else {
        Ok(h_service.into())
    }
}

pub(crate) fn query_service_status_process_info(
    h_service: HANDLE,
) -> Result<SERVICE_STATUS_PROCESS, WinapiError> {
    let mut service_status: SERVICE_STATUS_PROCESS = unsafe { std::mem::zeroed() };
    let mut bytes_needed: u32 = 0;
    let status = unsafe {
        QueryServiceStatusEx(
            h_service,
            SC_STATUS_PROCESS_INFO,
            &mut service_status as *mut SERVICE_STATUS_PROCESS as *mut u8,
            size_of::<SERVICE_STATUS_PROCESS>() as u32,
            &mut bytes_needed,
        )
    };

    if status == FALSE {
        Err(WinapiCallError::new("QueryServiceStatusEx").into())
    } else {
        Ok(service_status)
    }
}

pub(crate) fn create_service(
    h_manager: HANDLE,
    service_name: &str,
    manager_access: u32,
    service_cmd: &str,
) -> Result<SmartHandle, WinapiError> {
    let cservice_name = CString::new(service_name)?;
    let service_cmd = CString::new(service_cmd)?;
    let h_service = unsafe {
        CreateServiceA(
            h_manager,
            cservice_name.as_ptr() as *const u8,
            cservice_name.as_ptr() as *const u8,
            manager_access,
            SERVICE_WIN32_OWN_PROCESS,
            SERVICE_DEMAND_START,
            SERVICE_ERROR_NORMAL,
            service_cmd.as_ptr() as *const u8,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        )
    };

    if h_service.is_null() {
        Err(WinapiCallError::new("CreateServiceA").into())
    } else {
        Ok(h_service.into())
    }
}

pub(crate) fn change_service_config2(
    h_service: HANDLE,
    info_level: u32,
    info: &SERVICE_LAUNCH_PROTECTED_INFO,
) -> Result<(), WinapiError> {
    let status = unsafe {
        ChangeServiceConfig2A(
            h_service,
            info_level,
            info as *const SERVICE_LAUNCH_PROTECTED_INFO as *const c_void,
        )
    };
    if status == FALSE {
        Err(WinapiCallError::new("ChangeServiceConfig2").into())
    } else {
        Ok(())
    }
}

pub(crate) fn stop_service(h_service: HANDLE) -> Result<SERVICE_STATUS_PROCESS, WinapiError> {
    let mut service_status: SERVICE_STATUS_PROCESS = unsafe { std::mem::zeroed() };

    let status = unsafe {
        ControlService(
            h_service,
            SERVICE_CONTROL_STOP,
            &mut service_status as *mut SERVICE_STATUS_PROCESS as *mut SERVICE_STATUS,
        )
    };
    if status == FALSE {
        Err(WinapiCallError::new("ControlService").into())
    } else {
        Ok(service_status)
    }
}

pub(crate) fn delete_service(h_service: HANDLE) -> Result<(), WinapiError> {
    let status = unsafe { DeleteService(h_service) };

    if status == FALSE {
        Err(WinapiCallError::new("DeleteService").into())
    } else {
        Ok(())
    }
}
