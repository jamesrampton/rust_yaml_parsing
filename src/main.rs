use indoc::indoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    people: Vec<Person>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    age: u8,
    children: Option<Vec<Person>>,
    nicknames: Option<Vec<String>>,
}

fn main() -> Result<(), serde_yaml::Error> {
    let test_yaml: &str = indoc! {r#"
      people:
        - name: "Alice"
          age: 52
          children:
            - name: "Theodore"
              age: 12
              nicknames:
                - "Teddy"
        - name: "Robert"
          age: 26
          nicknames:
            - "Bob"
    "#};

    let data = serde_yaml::from_str::<Data>(test_yaml)?;
    println!("{}", serde_yaml::to_string(&data)?);
    Ok(())
}
