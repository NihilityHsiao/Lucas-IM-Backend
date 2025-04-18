use std::process::Command;

pub trait BuilderExt {
    fn with_serde(self, path: &[&str]) -> Self;
}

impl BuilderExt for tonic_build::Builder {
    fn with_serde(self, path: &[&str]) -> Self {
        path.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(serde::Serialize, serde::Deserialize)]")
        })
    }
}
fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .field_attribute("User.password", "#[serde(skip_serializing)]")
        .field_attribute("User.salt", "#[serde(skip_serializing)]")
        .with_serde(&["User"])
        .compile_protos(&["protos/user.proto"], &["protos"])
        .unwrap();

    // execute cargo fmt command
    Command::new("cargo").arg("fmt").output().unwrap();

    println!("cargo: rerun-if-changed=apps/user/rpc/protos/user.proto");
}
