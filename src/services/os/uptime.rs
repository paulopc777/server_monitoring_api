
pub fn get_uptime() -> Result<String, Box<dyn std::error::Error>> {
    let is_linux = cfg!(target_os = "linux");

    // run cmd command "uptime -s"
    if is_linux {
        let output = std::process::Command::new("uptime").arg("-s").output()?;

        let uptime = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(uptime)
    } else {
        // Obtém a data e hora do último boot no formato YYYY-MM-DD HH:MM:SS
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg("(Get-CimInstance Win32_OperatingSystem).LastBootUpTime.ToString('yyyy-MM-dd HH:mm:ss')")
            .output()?;
        let uptime = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        Ok(uptime)
    }
}
