use std::collections::BTreeMap;
use crate::common_util::SoftwareDescription;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BorgServerSideConfiguration {
    pub AppName: String,
    pub entryPoint: String,
    pub HostName: String,
    pub SshPort: i32,
    pub ServerName: String,
    pub coreNumber: u8,
    pub mem: String,
    pub UserName: String,
    pub LogDir: String,
    pub BorgBin: String,
    pub BorgInit: String,
    pub BorgCreate: String,
    pub BorgPrune: String,
    pub BorgList: String,
    pub BorgRepoPath: String,
    pub LocalDir: String,
    pub taskcmd: BTreeMap<String, String>,
    pub crons: BTreeMap<String, String>,
    pub BorgPrunePattern: String,
    pub Softwares: Vec<SoftwareDescription>,
    pub UninstallCommand: String,
    pub PackageDir: String,
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

        assert!(deserialized_conf.Softwares[0].PackageUrl.starts_with("https://"));
        assert_eq!(deserialized_conf.Softwares[0].LocalName, "");
    }
}