use sysinfo::System;
mod get_info;
mod gui;

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

// CONST information:
    get_info::get_const_info(); 
// RAM information:
    get_info::get_disk_info(&sys);
// CPU information:
    get_info::get_cpu_info(&sys);
// GPU information:
    get_info::display_gpu_info();
    gui::app::run_overlay();


}
