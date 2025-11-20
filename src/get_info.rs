use sysinfo::{Disks, System};
use whoami::{username, devicename, distro};
use std::process::Command;


pub struct GPUInfo {
    pub name: String,
    pub temperature: String,
    pub utilization: String,
    pub memory_used: String,
    pub memory_total: String, 
}


// Ф-я вывода инф о GPU    
pub fn display_gpu_info() -> String {
    match get_gpu_info() {
        Ok(info) => format!(
            "=== GPU Information ===\nName: {}\nTemperature: {}\nUsage: {}\nMemory Used: {}\nMemory Total: {}\n",
            info.name, info.temperature, info.utilization, info.memory_used, info.memory_total
        ),
        Err(e) => format!("Error: {}", e),
    }
}


// Вспомогательная функция для получения инфо (приватная)
fn get_gpu_info() -> Result<GPUInfo, Box<dyn std::error::Error>> {
    // Сначала пробуем nvidia-smi
    if let Ok(info) = try_nvidia_smi() {
        return Ok(info);
    }

    // Если NVIDIA не сработала, пробуем AMD radeon-smi
    if let Ok(info) = try_radeon_smi() {
        return Ok(info);
    }

    // Последний вариант — WMI
    try_wmi()
}

// NVIDIA: nvidia-smi
fn try_nvidia_smi() -> Result<GPUInfo, Box<dyn std::error::Error>> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=name,temperature.gpu,utilization.gpu,memory.used,memory.total")
        .arg("--format=csv,noheader,nounits")
        .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = result.trim().split(',').collect();

        if parts.len() >= 5 {
            // Парсим в числа с десятыми
            let temp: f32 = parts[1].trim().parse().unwrap_or(0.0);
            let util: f32 = parts[2].trim().parse().unwrap_or(0.0);
            
            return Ok(GPUInfo {
                name: parts[0].trim().to_string(),
                temperature: format!("{:.1} °C", temp),           
                utilization: format!("{:.1} %", util),           
                memory_used: format!("{} MB", parts[3].trim()),
                memory_total: format!("{} MB", parts[4].trim()),
            });
        }
    }

    Err("nvidia-smi failed or not installed".into())
}

// AMD: radeon-smi
fn try_radeon_smi() -> Result<GPUInfo, Box<dyn std::error::Error>> {
    let output = Command::new("radeon-smi")
        .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        
        let mut name = String::new();
        let mut temperature = String::new();
        let mut utilization = String::new();
        let memory_used = String::new();
        let memory_total = String::new();
        
        for line in result.lines() {
            if line.contains("GPU") && line.contains("model") {
                if let Some(val) = line.split(':').nth(1) {
                    name = val.trim().to_string();
                }
            } else if line.contains("Temperature") {
                if let Some(val) = line.split(':').nth(1) {
                    let temp = val.trim().replace("C", "").trim().to_string();
                    temperature = format!("{}°C", temp);
                }
            } else if line.contains("GPU Load") && let Some(val) = line.split(':').nth(1) {
                utilization = val.trim().to_string();
    }



        }

        return Ok(GPUInfo {
            name: if name.is_empty() { "AMD Radeon".to_string() } else { name },
            temperature,
            utilization,
            memory_used,
            memory_total,
        });
    }

    Err("radeon-smi failed or not installed".into())

}

// Fallback: WMI через PowerShell
fn try_wmi() -> Result<GPUInfo, Box<dyn std::error::Error>> {
    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg("Get-WmiObject win32_videocontroller | Select-Object Name, AdapterRAM | Format-List")
        .output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        
        let mut name = String::new();
        let mut memory_total = String::new();

        for line in result.lines() {
            if line.contains("Name") {
                if let Some(val) = line.split(':').nth(1) {
                    name = val.trim().to_string();
                }
            } else if line.contains("AdapterRAM") && let Some(val) = line.split(':').nth(1) {
                let bytes: u64 = val.trim().parse().unwrap_or(0);
                    memory_total = format!("{} MB", bytes / 1_000_000);
            }

        }
        

        return Ok(GPUInfo {
            name: if name.is_empty() { "Unknown GPU".to_string() } else { name },
            temperature: "N/A (требуется nvidia-smi или radeon-smi)".to_string(),
            utilization: "N/A".to_string(),
            memory_used: "N/A".to_string(),
            memory_total,
        });
    }

    Err("All GPU detection methods failed".into())
}


//########################################################################
// Ф-я вывода инф о диске    


pub fn get_disk_info(sys: &System) -> String {
    let mut info = format!(
        "RAM usage: {:.1} %\n",
        (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
    );

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let total = disk.total_space() as f64;
        let available = disk.available_space() as f64;
        let used = total - available;
        let used_percent = (used / total) * 100.0;
        let mount_point = disk.mount_point().to_string_lossy();

        info.push_str(&format!(
            "{} usage: {:.1} %\n",
            mount_point, used_percent
        ));
    }
    info
}


//########################################################################
// Ф-я вывода инф о GPU    


pub fn get_const_info() -> String {
    format!(
        "User: {}\nPC: {}\nOS: {}\n",
        username(),
        devicename(),
        distro()
    )
}



//########################################################################
// Ф-я вывода инф о CPU   


pub fn get_cpu_info(sys: &System) -> String {
    format!(
        "=== CPU Information ===\nName: {}\nUsage: {:.1} %\n",
        sys.cpus()[0].brand(),
        sys.global_cpu_usage()
    )
}



