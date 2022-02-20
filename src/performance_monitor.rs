use windows::Win32::System::SystemInformation;
use simple_error::SimpleError;
use std::sync::{Arc, Mutex};
use wmi::{COMLibrary, WMIConnection, Variant};
use std::collections::HashMap;

#[derive(Default)]
pub struct PerformanceStatistics {
   pub memory_usage: f32,
   pub cpu_usage: f32,
   pub cpu_temperature: f32,
   pub gpu_usage: f32,
   pub vram_usage: f32,
   pub gpu_temperature: f32,
}

// Periodically updates performance statistics. Uses openhardwaremonitor and wmi. Openhardwaremonitor must be running.
pub struct PerformanceMonitor {
    // We could avoid the mutex if there were atomic floats
    stats: Arc<Mutex<PerformanceStatistics>>,
    update_thread: std::thread::JoinHandle<()>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let stats = Arc::new(Mutex::new(Default::default()));
        PerformanceMonitor {
            stats: stats.clone(),
            update_thread: std::thread::spawn(move || {
                let com_con = COMLibrary::without_security().unwrap();
                let wmi_con = WMIConnection::with_namespace_path(r"root\OpenHardwareMonitor", com_con.into()).unwrap();
                loop {
                    update_stats(&stats, &wmi_con);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }),
        }
    }

    pub fn statistics(&self) -> Arc<Mutex<PerformanceStatistics>> {
        self.stats.clone()
    }
}

fn update_stats(stats: &std::sync::Arc<std::sync::Mutex<PerformanceStatistics>>, wmi_con: &WMIConnection) {
    let cpu = get_openhardwaremonitor_sensor_value(wmi_con, "CPU Total", "Load").unwrap_or_default() / 100.0;
    let cpu_temp = get_openhardwaremonitor_sensor_value(wmi_con, "CPU Package", "Temperature").unwrap_or_default();
    let ram = get_ram_usage();
    let gpu = get_openhardwaremonitor_sensor_value(wmi_con, "GPU Core", "Load").unwrap_or_default() / 100.0;
    let vram = get_openhardwaremonitor_sensor_value(wmi_con, "GPU Memory", "Load").unwrap_or_default() / 100.0;
    let gpu_temp = get_openhardwaremonitor_sensor_value(wmi_con, "GPU Core", "Temperature").unwrap_or_default();
    {
        let mut st = stats.lock().unwrap();
        st.memory_usage = ram;
        st.cpu_usage = cpu;
        st.cpu_temperature = cpu_temp;
        st.gpu_usage = gpu;
        st.vram_usage = vram;
        st.gpu_temperature = gpu_temp;
    }
}

fn get_openhardwaremonitor_sensor_value(wmi_con: &WMIConnection, sensor_name: &str, sensor_type: &str) -> Result<f32, SimpleError> {
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(format!("SELECT Value FROM Sensor WHERE Name='{}' AND SensorType='{}'", sensor_name, sensor_type)).unwrap();
    if results.len() == 0 {
        return Err(SimpleError::new(format!("No sensor '{}' of type '{}'.", sensor_name, sensor_type)));
    }
    let val = results[0].get("Value").unwrap();
    return match val {
        wmi::Variant::R4(fval) => Ok(*fval),
        _ => Err(SimpleError::new("The value was not a float.")),
    };
}

fn get_ram_usage() -> f32 {
    let mut mem_info: SystemInformation::MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    mem_info.dwLength = std::mem::size_of::<SystemInformation::MEMORYSTATUSEX>() as u32;
    unsafe {
        SystemInformation::GlobalMemoryStatusEx(&mut mem_info);
    }
    return mem_info.dwMemoryLoad as f32 / 100.0;
}
