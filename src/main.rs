use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    children: Option<Vec<String>>,
    #[serde(flatten)]
    extras: HashMap<String, String>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let test_yaml: &str = indoc! {r#"---
        - G1: "Test"
          children: [Sn1]
        - Sn1: "Lala"
        "#};

    let deser: Vec<Person> = serde_yaml::from_str(&test_yaml)?;
    println!("{:#?}", deser);
    // println!("{:?}", deser[0].supported_by.as_ref().unwrap()[0]);
    Ok(())
}
