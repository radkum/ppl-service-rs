#![feature(c_str_module)]

extern crate core;

mod service_manager;
use service_manager::{
    create_normal_service, create_protected_service, remove_protected_service, remove_service,
};

pub(crate) mod winapi;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 3 {
        print_help();
        return;
    }

    let service_file = "win-service.exe";
    let service_name = args[2].as_str();

    let ret = match args[1].as_str() {
        "create" => create_normal_service(service_file, service_name),
        "create_protected" => create_protected_service(service_file, service_name),
        "delete" => remove_service(service_name),
        "delete_protected" => remove_protected_service(service_name),
        _ => Ok(print_help()),
    };

    if let Err(e) = ret {
        eprintln!("{}", e);
    }
}

fn print_help() {
    println!("Use: ppl-install.exe <create|create_protected|remove> <service_name>");
}
