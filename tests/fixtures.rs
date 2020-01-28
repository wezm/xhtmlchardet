extern crate toml;
extern crate xhtmlchardet;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};

fn read_config() -> toml::Value {
    let mut file = File::open("tests/fixtures.toml")
        .ok()
        .expect("Unable to open tests/fixtures.toml");
    let mut toml = String::new();
    file.read_to_string(&mut toml)
        .ok()
        .expect("Error reading config file");
    let config: toml::Value = toml.parse().ok().expect("Error parsing config file");
    config
}

#[test]
fn test_fixtures() {
    let config = read_config();

    let tests = config
        .lookup("fixtures")
        .expect("no fixtures in file")
        .as_slice()
        .expect("fixtures is not an array");

    // Map the filename to the output charset
    let mut expected: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut actual: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for value in tests {
        let test = value.as_table().expect("invalid test");
        let expected_set: Vec<String> = test["charset"]
            .as_slice()
            .unwrap()
            .to_vec()
            .iter()
            .map(|item| item.as_str().unwrap().to_string())
            .collect();
        let variant = test["variant"].as_str().unwrap();
        let hint = test
            .get("hint")
            .map(|hint| hint.as_str().unwrap().to_string());
        let path = format!("tests/{}-{}.txt", expected_set[0], variant);

        expected.insert(path.to_string(), expected_set);

        let mut file = File::open(&path)
            .ok()
            .expect(&format!("Unable to open {}", path));
        let actual_charset = xhtmlchardet::detect(&mut file, hint.clone());
        actual.insert(path.to_string(), actual_charset.unwrap());
    }

    // Verify the results
    let mut passed = 0;
    let mut f = std::io::stderr();
    for (test, result) in expected.iter() {
        if *result != actual[test] {
            f.write(format!("FAIL {}: {:?} != {:?}\n", test, actual[test], result).as_bytes())
                .unwrap();
        } else {
            passed += 1;
            // f.write(format!("PASS {}: {:?}\n", test, actual[test]).as_bytes());
        }
    }

    assert_eq!(passed, expected.len());
}
