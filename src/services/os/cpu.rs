use sysinfo::System;

pub struct CpuInfo {
    pub total_cpus: u32,
    pub total_cpu_usage: u32,
    pub cores_usage: String,
}

pub fn print_cpu_info(sys: &System) -> CpuInfo {
    let total_cpus: usize = sys.cpus().len();
    let mut sys_new = System::new();
    let mut total_cpu_usage = 0.0;
    let _ = sys_new.refresh_cpu_usage();

    let mut cores_usage = Vec::new();

    for i in [1, 2] {
        sys_new.refresh_cpu_usage(); // Refreshing CPU usage.
        for cpu in sys_new.cpus() {
            if !(i != 1) {
                continue;
            }
            cores_usage.push(cpu.cpu_usage());
            total_cpu_usage += cpu.cpu_usage();
        }

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }

    CpuInfo {
        total_cpus: total_cpus as u32,
        total_cpu_usage: total_cpu_usage as u32 / total_cpus as u32,
        cores_usage: format!("{:?}", cores_usage),
    }
}
