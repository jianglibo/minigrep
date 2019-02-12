use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlServerSideConfiguration {
    app_name: String,
    mysql_version: String,
    server_name: String,
    core_number: u8,
    mem: String,
    mysql_user: String,
    mysql_password: String,
    client_bin: String,
    dump_bin: String,
    dump_filename: String,
    mysql_admin_bin: String,
    mysql_log_file: String,
    local_dir: String,
    log_dir: String,
    task_cmd: BTreeMap<String, String>,
    crons: BTreeMap<String, String>,
    dump_prune_pattern: String,
    softwares: Vec<BTreeMap<String, String>>,
    start_command: String,
    stop_command: String,
    restart_command: String,
    status_command: String,
    uninstall_command: String,
    script_dir: String,
    package_dir: String,
    entry_point: String,
}

#[cfg(test)]
mod tests {
    use crate::fixture_util::{get_fixture_file};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_configuration() {
        let p = get_fixture_file(&["mysql", "mysql_configuration.1.yml"], true);
        let mut file = File::open(p.unwrap()).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let deserialized_conf: super::MysqlServerSideConfiguration = serde_yaml::from_str(&buf).unwrap();

        assert_eq!(deserialized_conf.app_name, "mysql");
    }
}