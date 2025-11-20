use sysinfo::System;
mod get_info;
mod gui;


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let const_info = get_info::get_const_info();
    
    let _ = gui::app::run_overlay(const_info);
}