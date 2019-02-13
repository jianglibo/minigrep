use std::collections::BTreeMap;
use crate::common_util::SoftwareDescription;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BorgServerSideConfiguration {
    pub app_name: String,
    pub entry_point: String,
    pub host_name: String,
    pub ssh_port: i32,
    pub server_name: String,
    pub core_number: u8,
    pub mem: String,
    pub user_name: String,
    pub log_dir: String,
    pub borg_bin: String,
    pub borg_init: String,
    pub borg_create: String,
    pub borg_prune: String,
    pub borg_list: String,
    pub borg_repo_path: String,
    pub local_dir: String,
    pub task_cmd: BTreeMap<String, String>,
    pub crons: BTreeMap<String, String>,
    pub borg_prune_pattern: String,
    pub softwares: Vec<SoftwareDescription>,
    pub uninstall_command: String,
    pub package_dir: String,
}

#[cfg(test)]
mod tests {
    use crate::fixture_util::{get_fixture_file};
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_configuration() {
        let p = get_fixture_file(&["borg", "borg_configuration.1.yml"], true);
        let mut file = File::open(p.unwrap()).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let deserialized_conf: super::BorgServerSideConfiguration = serde_yaml::from_str(&buf).unwrap();

        assert!(deserialized_conf.softwares[0].package_url.starts_with("https://"));
        assert_eq!(deserialized_conf.softwares[0].local_name, "");
    }
}