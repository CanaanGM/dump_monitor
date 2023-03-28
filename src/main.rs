use std::fmt::Error;

use sysinfo::{CpuExt, NetworkExt, RefreshKind, System, SystemExt, CpuRefreshKind};

fn main() -> Result<(), Error> {
    let mut system = System::new_all();

    loop {
        system.refresh_specifics(
            RefreshKind::new()
                .with_memory()
                .with_networks()
                .with_cpu(CpuRefreshKind::everything())
        );

        println!("CPU usage: {}%", system.global_cpu_info().cpu_usage());

        let total_memory = system.total_memory() as f32 / 1024.0 / 1024.0;
        let used_memory = system.used_memory() as f32 / 1024.0 / 1024.0;
        let networks = system
            .networks()
            .into_iter()
            .find(|nw| *nw.0 == "Ethernet".to_string())
            .unwrap();

        println!(
            "Memory usage: {:.2} MB / {:.2} MB ({:.2}%)",
            used_memory,
            total_memory,
            used_memory / total_memory * 100.0
        );

        println!(
            "Download: {download:.4}kb\nUpload: {upload:.4}kb",
            download =    bit_to_kbit(networks.1.total_received()) ,
            upload   =    bit_to_kbit(networks.1.total_transmitted()) 
        );

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

fn bit_to_kbit(bit: u64) -> f64 {
    let  b  = bit as f64 * 0.000001;
     b * 0.000125
}