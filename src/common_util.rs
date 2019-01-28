
const LINE_START: &str = "for-easyinstaller-client-use-start";
const LINE_END: &str = "for-easyinstaller-client-use-end";

pub fn send_string_to_client(str_content: &str) {
    println!("{}", LINE_START);
    println!("{}", str_content);
    println!("{}", LINE_END);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareDescription {
    pub PackageUrl: String,
    pub LocalName: String,
}