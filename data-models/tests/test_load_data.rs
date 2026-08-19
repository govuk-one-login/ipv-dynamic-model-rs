use data_models::models::cri::Cri;
use serde::Deserialize;
use std::fs::File;

#[test]
fn test_load_data() -> anyhow::Result<()> {
    println!("Loading test-data");
    let mut cris = Vec::new();
    for document in yaml_serde::Deserializer::from_reader(File::open("../test-data/test.yaml")?) {
        cris.push(Cri::deserialize(document)?);
    }
    for cri in cris {
        println!("{cri:#?}");
    }
    Ok(())
}
