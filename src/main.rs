use questdb_confstr::parse_conf_str;

pub fn ping_questdb(conf: &str) -> Result<(), String> {
    let config =
        parse_conf_str(conf).map_err(|e| format!("Failed to parse config string: {}", e))?;

    let service = config.service();
    if service != "http" && service != "https" {
        return Err(format!("Unsupported service type: {}", service));
    }

    let addr = config
        .get("addr")
        .ok_or_else(|| "Missing 'addr' field in configuration".to_string())?;

    let url = format!("{}://{}/ping", service, addr);

    match ureq::get(&url).call() {
        Ok(_response) => Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}

fn main() {
    let conf = "http::addr=localhost:9000;";
    match ping_questdb(conf) {
        Ok(_) => println!("QuestDB is alive!"),
        Err(e) => eprintln!("Failed to ping QuestDB: {}", e),
    }
}
