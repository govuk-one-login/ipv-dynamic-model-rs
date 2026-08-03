use ipv_dynamic_model_rs::models::cri::Cri;
use serde::Deserialize;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    println!("Loading data");
    let mut cris = Vec::new();
    for document in yaml_serde::Deserializer::from_reader(File::open("data/test.yaml")?) {
        cris.push(Cri::deserialize(document)?);
    }
    for cri in cris {
        println!("{cri:#?}");
    }
    Ok(())
}
