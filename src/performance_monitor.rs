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
   pub gpu_temperature: u32,
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
                    // Updating cpu takes one second (because of how typeperf works), so this will loop every second
                    update_stats(&stats, &wmi_con);
                }
            }),
        }
    }

    pub fn statistics(&self) -> Arc<Mutex<PerformanceStatistics>> {
        self.stats.clone()
    }
}

fn update_stats(stats: &std::sync::Arc<std::sync::Mutex<PerformanceStatistics>>, wmi_con: &WMIConnection) {
    let cpu = get_cpu_usage();
    let ram = get_ram_usage();
    let (gpu, vram, temperature) = match get_gpu_stats() {
        Ok(val) => val,
        Err(err) => { println!("{}", err); (0.0, 0.0, 0) },
    };
    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT Value FROM Sensor WHERE Name='CPU Package' AND SensorType='Temperature'").unwrap();
    assert_eq!(results.len(), 1);
    let val = results[0].get("Value").unwrap();
    let cpu_temp = match val {
        wmi::Variant::R4(fval) => fval,
        _ => panic!("NAUUR"),
    };
    {
        let mut st = stats.lock().unwrap();
        st.memory_usage = ram;
        st.cpu_usage = cpu;
        st.cpu_temperature = *cpu_temp;
        st.gpu_usage = gpu;
        st.vram_usage = vram;
        st.gpu_temperature = temperature;
    }
}

fn get_cpu_usage() -> f32 {
    return (get_counter_value::<f32>(r"\Processor Information(_Total)\% Processor Utility") / 100.0).min(1.0);
}
fn get_ram_usage() -> f32 {
    let mut mem_info: SystemInformation::MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    mem_info.dwLength = std::mem::size_of::<SystemInformation::MEMORYSTATUSEX>() as u32;
    unsafe {
        SystemInformation::GlobalMemoryStatusEx(&mut mem_info);
    }
    return mem_info.dwMemoryLoad as f32 / 100.0;
}

/**
 * Gets gpu (3d) usage, vram usage and gpu temperature
 */
fn get_gpu_stats() -> Result<(f32, f32, u32), SimpleError> {
    let output = std::process::Command::new("nvidia-smi.exe").args(["-q", "-d", "UTILIZATION,MEMORY,TEMPERATURE"]).output();
    match output {
        Ok(out) => {
            let stdout = std::string::String::from_utf8_lossy(out.stdout.as_slice());
            let engine_usage = {
                let util_regex = regex::Regex::new(r"Gpu\s+:\s+(\d+)\s%").expect("Regex failed to compile");
                let value = util_regex.captures(&stdout).unwrap().get(1).unwrap();
                value.as_str().parse::<f32>().unwrap() / 100.0
            };
            let vram_usage = {
                let total_regex = regex::Regex::new(r"Total\s+:\s+(\d+)\sMiB").expect("Regex failed to compile");
                let total = total_regex.captures(&stdout).unwrap().get(1).unwrap();
                let total_val = total.as_str().parse::<f32>().unwrap();
                let used_regex = regex::Regex::new(r"Used\s+:\s+(\d+)\sMiB").expect("Regex failed to compile");
                let used = used_regex.captures(&stdout).unwrap().get(1).unwrap();
                let used_val = used.as_str().parse::<f32>().unwrap();
                used_val / total_val
            };
            let temperature = {
                let temp_regex = regex::Regex::new(r"GPU Current Temp\s+:\s+(\d+)\sC").expect("Regex failed to compile");
                let value = temp_regex.captures(&stdout).unwrap().get(1).unwrap();
                value.as_str().parse().unwrap()
            };

            Ok((engine_usage, vram_usage, temperature))
        },
        Err(err) => Err(SimpleError::new(format!("Fetching GPU information failed. Make sure nvidia-smi.exe is in the PATH. ({})", err))),
    }
}

fn get_counter_value<T>(counter: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let output = std::process::Command::new("typeperf").arg(counter).args(["-sc", "1"]).output().unwrap();
    let stdout = std::string::String::from_utf8_lossy(output.stdout.as_slice());
    let regex = regex::Regex::new("\"(\\d*\\.\\d*)\"").expect("Regex failed to compile");
    let captures = regex.captures(&stdout).unwrap();
    let value = captures.get(1).unwrap();
    return value.as_str().parse::<T>().unwrap();
}