use chrono::{DateTime, Duration, Local, Utc};
use procfs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};

pub fn get_cpu_info() -> String {
    let mut system = System::new_all();
    system.refresh_cpu();

    let cpus = system.get_processors();

    let mut result = String::new();
    result += "CPU Information:\n";

    for cpu_info in cpus {
        result += &format!("Name: {}\n", cpu_info.get_name());
        result += &format!("Brand: {}\n", cpu_info.get_brand());
        result += &format!("Frequency: {} MHz\n", cpu_info.get_frequency());
        result += &format!("Vendor ID: {}\n", cpu_info.get_vendor_id());
        result += &format!("Cpu usage: {}%\n", cpu_info.get_cpu_usage());
        result += "\n";
    }

    result
}

pub fn get_ram_info() -> String {
    let system = System::new_all();
    let total_ram = system.get_total_memory();
    let available_ram = system.get_available_memory();
    let used_ram = total_ram - available_ram;
    let ram_info = format!(
        "Memory Information\nTotal RAM: {} bytes\nAvailable RAM: {} bytes\nUsed RAM: {} bytes",
        total_ram, available_ram, used_ram
    );
    ram_info
}

pub fn get_storage_info() -> String {
    let mut system = System::new_all();
    system.refresh_all();

    let disks = system.get_disks();

    let mut result = String::new();
    result += "Disk Information:\n\n";

    for disk in disks {
        result += &format!("Device: {:?}\n", disk.get_type());
        result += &format!("Type: {:?}\n", disk.get_type());
        result += &format!("Total Space: {} bytes\n", disk.get_total_space());
        result += &format!("Available Space: {} bytes\n", disk.get_available_space());
        result += "\n";
    }

    result
}

pub fn get_uptime() -> String {
    let mut system = System::new_all();
    system.refresh_system();

    let uptime = Duration::seconds(system.get_uptime() as i64);
    let uptime_formatted = format!(
        "{} days, {} hours, {} minutes, {} seconds",
        uptime.num_days(),
        uptime.num_hours() % 24,
        uptime.num_minutes() % 60,
        uptime.num_seconds() % 60
    );

    let mut result = String::new();
    result += &format!("Uptime: {}\n", uptime_formatted);

    result
}

pub fn get_loaded_modules() -> String {
    let modules = procfs::modules();
    let mut result = String::new();

    result += "Loaded Modules:\n";
    match modules {
        Ok(modules) => {
            for module in modules {
                result += &format!("Name: {}\n", module.1.name);
                result += &format!("Size: {}\n", module.1.size);
                result += &format!("Used By: {:?}\n", module.1.used_by);
                result += "\n";
            }
        }
        Err(err) => {
            result += &format!("Error: {}\n", err);
        }
    }
    result
}

fn generate_file_path() -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let logs_dir = current_dir.join("logs");
    std::fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");

    let current_time: DateTime<Local> = Utc::now().into();
    let timestamp = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_name = format!("{}.log", timestamp);

    logs_dir.join(file_name)
}

pub fn generate_log_file() -> PathBuf {
    let file_path = generate_file_path();
    println!("Generated file path: {:?}", file_path);

    let mut file = File::create(&file_path).expect("Failed to create the file");

    let content = &format!(
        "{}\n{}\n{}\n{}\n{}",
        get_uptime(),
        get_cpu_info(),
        get_ram_info(),
        get_storage_info(),
        get_loaded_modules(),
    );
    file.write_all(content.as_bytes())
        .expect("Failed to write to the file");

    println!("File '{:?}' has been created.", file_path);
    return file_path;
}
