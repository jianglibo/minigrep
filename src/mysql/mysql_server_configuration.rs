use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MysqlServerSideConfiguration {
    AppName: String,
    MysqlVersion: String,
    ServerName: String,
    coreNumber: u8,
    mem: String,
    MysqlUser: String,
    MysqlPassword: String,
    ClientBin: String,
    DumpBin: String,
    DumpFilename: String,
    MysqlAdminBin: String,
    MysqlLogFile: String,
    LocalDir: String,
    LogDir: String,
    taskcmd: BTreeMap<String, String>,
    crons: BTreeMap<String, String>,
    DumpPrunePattern: String,
    Softwares: Vec<BTreeMap<String, String>>,
    StartCommand: String,
    StopCommand: String,
    RestartCommand: String,
    StatusCommand: String,
    UninstallCommand: String,
    ScriptDir: String,
    PackageDir: String,
    EntryPoint: String,
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

        assert_eq!(deserialized_conf.AppName, "mysql");
    }
}