use sysinfo::System;
mod get_info;
mod gui;


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let const_info = get_info::get_const_info();
    let disk_info = get_info::get_disk_info(&sys);
    let cpu_info = get_info::get_cpu_info(&sys);
    let gpu_info = get_info::display_gpu_info();

    let _ = gui::app::run_overlay(const_info, disk_info, cpu_info, gpu_info);
}