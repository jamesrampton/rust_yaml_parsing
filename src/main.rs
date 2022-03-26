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
        - bobby: "Parent"
          children: [Linda]
        - Linda: "Child"
        "#};

    let people: Vec<Person> = serde_yaml::from_str(&test_yaml)?;
    println!("{}", people[0].children.as_ref().unwrap()[0]);
    Ok(())
}
