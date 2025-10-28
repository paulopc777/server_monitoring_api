use sysinfo::System;

pub fn print_memory_info(sys: &System) -> [u64; 3] {
    let total_memory = sys.total_memory() / 1024 / 1024;
    let used_memory = sys.used_memory() / 1024 / 1024;
    let free_memory = sys.free_memory() / 1024 / 1024;
    return [total_memory, used_memory, free_memory];
}
