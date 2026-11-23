// Dynamic, read-only menu telemetry. Never use these values for authorization.
//
// Credits:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuTelemetry {
    pub date: String,
    pub host: String,
    pub operating_system: String,
    pub process_uptime: String,
    pub server_memory: String,
    pub control_memory: String,
    pub version: &'static str,
}

impl MenuTelemetry {
    pub fn capture() -> Self {
        Self {
            date: local_date(),
            host: read_hostname(),
            operating_system: format!("{} ({})", std::env::consts::OS, std::env::consts::ARCH),
            process_uptime: process_uptime(),
            server_memory: server_memory(),
            control_memory: control_memory(),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}

fn local_date() -> String {
    let output = Command::new("date").arg("+%d/%m/%Y").output();
    output
        .ok()
        .filter(|result| result.status.success())
        .and_then(|result| String::from_utf8(result.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| value.len() == 10 && value.chars().all(|c| c.is_ascii_digit() || c == '/'))
        .unwrap_or_else(|| "No disponible".to_owned())
}

fn read_hostname() -> String {
    #[cfg(target_os = "linux")]
    {
        fs::read_to_string("/proc/sys/kernel/hostname")
            .ok()
            .map(|host| host.trim().to_owned())
            .filter(|host| {
                !host.is_empty()
                    && host.len() <= 63
                    && host
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || b"-._".contains(&byte))
            })
            .unwrap_or_else(|| "No disponible".to_owned())
    }
    #[cfg(not(target_os = "linux"))]
    {
        "No disponible".to_owned()
    }
}

fn memory_kb(key: &str, body: &str) -> Option<u64> {
    body.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        if name != key {
            return None;
        }
        value.split_whitespace().next()?.parse().ok()
    })
}

fn server_memory() -> String {
    let Ok(contents) = fs::read_to_string("/proc/meminfo") else {
        return "No disponible".to_owned();
    };
    let (Some(total), Some(available)) = (
        memory_kb("MemTotal", &contents),
        memory_kb("MemAvailable", &contents),
    ) else {
        return "No disponible".to_owned();
    };
    if total == 0 || available > total {
        return "No disponible".to_owned();
    }
    let used = total - available;
    format!(
        "{:.2} GiB / {:.2} GiB ({:.1}%)",
        used as f64 / 1_048_576.0,
        total as f64 / 1_048_576.0,
        used as f64 * 100.0 / total as f64
    )
}

fn control_memory() -> String {
    let Ok(contents) = fs::read_to_string("/proc/self/status") else {
        return "No disponible".to_owned();
    };
    memory_kb("VmRSS", &contents)
        .map(|kb| format!("{:.1} MiB", kb as f64 / 1024.0))
        .unwrap_or_else(|| "No disponible".to_owned())
}

fn process_uptime() -> String {
    #[cfg(target_os = "linux")]
    {
        let start_ticks = fs::read_to_string("/proc/self/stat")
            .ok()
            .and_then(|s| s.rsplit_once(") ").map(|(_, fields)| fields.to_owned()))
            .and_then(|fields| {
                fields
                    .split_whitespace()
                    .nth(19)
                    .and_then(|value| value.parse::<u64>().ok())
            });
        let boot_uptime = fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok());
        let ticks_per_second = Command::new("getconf")
            .arg("CLK_TCK")
            .output()
            .ok()
            .filter(|result| result.status.success())
            .and_then(|result| String::from_utf8(result.stdout).ok())
            .and_then(|value| value.trim().parse::<u64>().ok());
        if let (Some(ticks), Some(boot), Some(hz)) = (start_ticks, boot_uptime, ticks_per_second) {
            if hz > 0 && boot.is_finite() && boot >= ticks as f64 / hz as f64 {
                let seconds = (boot - ticks as f64 / hz as f64) as u64;
                return format!(
                    "{}d {}h {}m {}s",
                    seconds / 86_400,
                    seconds / 3_600 % 24,
                    seconds / 60 % 60,
                    seconds % 60
                );
            }
        }
    }
    "No disponible".to_owned()
}

#[cfg(test)]
mod tests {
    use super::{memory_kb, MenuTelemetry};

    #[test]
    fn parses_memory_fields_without_guessing() {
        assert_eq!(
            memory_kb("MemTotal", "MemTotal: 4096 kB\nMemAvailable: 2048 kB"),
            Some(4096)
        );
        assert_eq!(memory_kb("MemFree", "MemTotal: 4096 kB"), None);
    }

    #[test]
    fn snapshot_supplies_real_build_version_without_rendering_design() {
        let snap = MenuTelemetry::capture();
        assert_eq!(snap.version, env!("CARGO_PKG_VERSION"));
        assert!(!snap.date.is_empty());
        assert!(!snap.operating_system.is_empty());
    }
}
