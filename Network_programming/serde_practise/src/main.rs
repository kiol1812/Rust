#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    workers: u64,
    ignoere: bool,
    auth_server: Option<String>
}

fn main() {
    let config = ServerConfig {
        workers: 100,
        ignoere: false,
        auth_server: Some("auth.server.io".to_string())
    };
    println!("To and from YAML");
    let serialized = serde_yaml::to_string(&config).unwrap();
    println!("{}", serialized);
    let deserialized:ServerConfig = serde_yaml::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);

    println!("-----------------------");

    println!("To and from JSOM");
    let serialized = serde_json::to_string(&config).unwrap();
    println!("{}", serialized);
    let deserialized:ServerConfig = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
}
