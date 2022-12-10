use windows::Win32::System::SystemInformation;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use wmi::{COMLibrary, WMIConnection, Variant};
use std::collections::HashMap;

// i7-12700k. We could probably fetch this information from the system.
const P_CORES: u32 = 8;

#[derive(Default)]
pub struct PerformanceStatistics {
   pub memory_usage: f32,
   pub cpu_usage_group_1: f32,
   pub cpu_usage_group_2: f32,
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
    let (cpu_1, cpu_2) = get_cpu_usage(wmi_con).unwrap();
    let cpu_temp = get_openhardwaremonitor_sensor_value(wmi_con, "CPU Package", "Temperature").unwrap_or_default();
    let ram = get_ram_usage();
    let gpu = get_openhardwaremonitor_sensor_value(wmi_con, "GPU Core", "Load").unwrap_or_default() / 100.0;
    let vram = get_openhardwaremonitor_sensor_value(wmi_con, "GPU Memory", "Load").unwrap_or_default() / 100.0;
    let gpu_temp = get_openhardwaremonitor_sensor_value(wmi_con, "GPU Core", "Temperature").unwrap_or_default();
    {
        let mut st = stats.lock().unwrap();
        st.memory_usage = ram;
        st.cpu_usage_group_1 = cpu_1;
        st.cpu_usage_group_2 = cpu_2;
        st.cpu_temperature = cpu_temp;
        st.gpu_usage = gpu;
        st.vram_usage = vram;
        st.gpu_temperature = gpu_temp;
    }
}

fn get_openhardwaremonitor_sensor_value(wmi_con: &WMIConnection, sensor_name: &str, sensor_type: &str) -> anyhow::Result<f32> {
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query(format!("SELECT Value FROM Sensor WHERE Name='{}' AND SensorType='{}'", sensor_name, sensor_type))?;
    if results.len() == 0 {
        return Err(anyhow::anyhow!(format!("No sensor '{}' of type '{}'.", sensor_name, sensor_type)));
    }
    let val = results[0].get("Value").ok_or(anyhow::anyhow!(format!("Found no value for sensor '{}'", sensor_name)))?;
    return match val {
        wmi::Variant::R4(fval) => Ok(*fval),
        x => Err(anyhow::anyhow!("Invalid WMI value. Expected float, found {:?}", x)),
    };
}

/// Returns CPU usage separated by P-cores and E-cores
fn get_cpu_usage(wmi_con: &WMIConnection) -> anyhow::Result<(f32, f32)> {
    let get_core_usage = |cores: Range<u32>| {
        let (core_count, usage_sum) = cores.map(|core| {
            get_openhardwaremonitor_sensor_value(wmi_con, &format!("CPU Core #{}", core + 1), "Load").map(|val| val / 100.0)
        })
        .take_while(Result::is_ok).map(Result::unwrap)
        .fold((0.0, 0.0), |(count, sum), val| (count + 1.0, sum + val));
        usage_sum / core_count
    };
    let usage_p = get_core_usage(0..P_CORES);
    let usage_e = get_core_usage(P_CORES..(u32::MAX));
    Ok((usage_p, usage_e))
}

fn get_ram_usage() -> f32 {
    let mut mem_info: SystemInformation::MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    mem_info.dwLength = std::mem::size_of::<SystemInformation::MEMORYSTATUSEX>() as u32;
    unsafe {
        SystemInformation::GlobalMemoryStatusEx(&mut mem_info);
    }
    return mem_info.dwMemoryLoad as f32 / 100.0;
}
