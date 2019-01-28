use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::borg::borg_server_configuration::BorgServerSideConfiguration;

#[derive(Debug)]
pub struct BorgInvoker {
    configuration: BorgServerSideConfiguration,
}


impl BorgInvoker {
    // We return a Result only if the situation is recoverable!!!
    pub fn load_from_config_file<T: AsRef<Path>>(file: T) -> BorgInvoker {
        let mut file = File::open(file).unwrap();
        // RAII
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        // let deserialized_conf_op: BorgServerSideConfiguration = serde_yaml::from_str(&buf);
        // match deserialized_conf_op {
        //     Some(doc) => Ok(BorgInvoker{configuration: doc}),
        //     _ => ()
        // }
        let deserialized_conf: BorgServerSideConfiguration = serde_yaml::from_str(&buf).unwrap();
        // Multi document support, doc is a yaml::Yaml
        // let doc = &(docs[0]);
        BorgInvoker {
            configuration: deserialized_conf
        }
    }

    fn install_borg(self) -> &'static str {
        if std::path::Path::new(&self.configuration.BorgBin).exists() {
            "AlreadyIntalled"
        } else {
            ""
        }
    }

    fn new_borg_archive(self) {
        
    }

}