use std::{env, fs};
pub fn get_ibus_address() -> String {
    let ibus_address = env::var("IBUS_ADDRESS");

    if let Ok(ibus_address) = ibus_address {
        return ibus_address;
    }

    let config_path = get_ibus_config_file();
    let config_file = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("cannot open config file {}", config_path));

    for line in config_file.lines() {
        let line = line.trim();
        if let Some(ibus_address) = line.strip_prefix("IBUS_ADDRESS=") {
            return ibus_address.to_string();
        }
    }

    panic!("Cannot get the ibus address");
}

fn get_ibus_config_file() -> String {
    format!(
        "{}/ibus/bus/{}-{}",
        get_config_dir(),
        get_machine_id(),
        get_host_and_display()
    )
}

fn get_config_dir() -> String {
    let config_dir = env::var("XDG_CONFIG_HOME");

    if let Ok(config_dir) = config_dir {
        return config_dir;
    }

    env::var("HOME").expect("Cannot get the home directory") + "/.config"
}

fn get_machine_id() -> String {
    let machine_id = fs::read_to_string("/var/lib/dbus/machine-id")
        .or_else(|_| fs::read_to_string("/etc/machine-id"))
        .expect("Cannot get the machine id ");

    machine_id.trim().to_string()
}

fn get_host_and_display() -> String {
    if let Ok(display) = env::var("WAYLAND_DISPLAY") {
        return format!("unix-{}", display);
    }

    if let Ok(display) = env::var("DISPLAY") {
        let Some((mut hostname, right)) = display.split_once(':') else {
            panic!("Wrong DISPLAY format");
        };

        let Some((mut display_number, _)) = right.split_once('.') else {
            panic!("Wrong DISPLAY format");
        };

        if hostname.is_empty() {
            hostname = "unix";
        }

        if display_number.is_empty() {
            display_number = "0";
        }

        return format!("{}-{}", hostname, display_number);
    }

    "unix-0".to_string()
}

#[cfg(test)]
mod test {
    use crate::utils::get_ibus_address;

    #[test]
    fn address() {
        println!("{}", get_ibus_address());
    }
}
