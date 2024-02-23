use anyhow;
use std::collections::HashMap;
use std::fs;

const THERMAL_ZONE_ROOT: &str = "/sys/class/thermal";

fn read_temperature_from_file(file_name: &str) -> anyhow::Result<u32> {
    let contents = fs::read_to_string(file_name)?;

    let temp = contents.parse::<u32>()?;
    Ok(temp)
}

fn get_thermal_zone_paths() -> Result<Vec<String>, std::io::Error> {
    let mut paths = Vec::new();
    let dir_entries = fs::read_dir(THERMAL_ZONE_ROOT)?;

    for entry in dir_entries {
        let entry = match entry {
            Ok(v) => v,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() && path.to_string_lossy().contains("thermal_zone") {
            paths.push(path.file_name().unwrap().to_string_lossy().into_owned());
        }
    }

    Ok(paths)
}

fn main() -> anyhow::Result<()> {
    //first, get a list of all thermal zones available on machine (assuming they're 'thermal_zone*' dirs in THERMAL_ZONE_ROOT)
    let thermal_zones = get_thermal_zone_paths()?;
    let mut tmzmap = HashMap::<String, u32>::new();
    for thermal_zone in thermal_zones {
        let path = format!("{}/{}", THERMAL_ZONE_ROOT, thermal_zone);
        dbg!();
        let temp = read_temperature_from_file(&path)?;
        tmzmap.insert(path, temp);
    }

    dbg!(tmzmap);
    Ok(())
}
