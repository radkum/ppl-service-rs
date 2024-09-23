#![feature(c_str_module)]

extern crate core;

mod service_manager;

use service_manager::{
    create_normal_service, create_protected_service, remove_protected_service, remove_service,
};

pub(crate) mod winapi;

const DEFAULT_SERVICE_PATH: &str = "win-service.exe";

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        print_help();
        return;
    }

    let service_file = if args.len() == 4 {
        let arg_3 = args[3].clone();
        if let Some(s) = arg_3.strip_prefix("binPath=") {
            s.to_string()
        } else {
            eprintln!("Third argument has to start from \"binPath=\"");
            print_help();
            return;
        }
    } else {
        DEFAULT_SERVICE_PATH.to_string()
    };

    let service_name = args[2].as_str();

    let ret = match args[1].as_str() {
        "create" => create_normal_service(service_file.as_str(), service_name),
        "create_protected" => create_protected_service(service_file.as_str(), service_name),
        "delete" => remove_service(service_name),
        "delete_protected" => remove_protected_service(service_name),
        _ => Ok(print_help()),
    };

    if let Err(e) = ret {
        eprintln!("{}", e);
    }
}

fn print_help() {
    println!("Usage:");
    println!("\tppl-install.exe <create|create_protected|remove> <service_name>");
    println!("\tor");
    println!(
        "\tppl-install.exe <create|create_protected|remove> <service_name> \
         bin_path=<path_to_service>\n"
    );
    println!("In the first case path_to_service is default, and means \"win-service.exe\"");
}
