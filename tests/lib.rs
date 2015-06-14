extern crate toml;
extern crate xhtmlchardet;

use std::io;

use std::io::{Read,Write};
use std::fs::File;
use std::collections::BTreeMap;

fn read_config() -> toml::Value {
    // let path = Path::new(&config_arg);
    let mut file = File::open("tests/fixtures.toml")
        .ok()
        .expect("Unable to open tests/fixtures.toml");
    let mut toml = String::new();
    file.read_to_string(&mut toml).ok().expect("Error reading config file");
    let config: toml::Value = toml.parse().ok().expect("Error parsing config file");
    config
}

#[test]
fn test_fixtures() {
    let config = read_config();

    let tests = config.lookup("fixtures")
        .expect("no fixtures in file")
        .as_slice().expect("fixtures is not an array");

    // Map the filename to the output charset
    let mut expected: BTreeMap<String, Option<String>> = BTreeMap::new();
    let mut actual: BTreeMap<String, Option<String>> = BTreeMap::new();

    for value in tests {
        let test = value.as_table().expect("invalid test");
        let expected_charset = test["charset"].as_str().unwrap();
        let variant = test["variant"].as_str().unwrap();
        let path = format!("tests/{}-{}.txt", expected_charset, variant);

        expected.insert(path.to_string(), Some(expected_charset.to_string()));

        let mut file = File::open(&path).ok().expect(&format!("Unable to open {}", path));
        let actual_charset = xhtmlchardet::detect(&mut file, None);
        actual.insert(path.to_string(), actual_charset);
    }

    // Verify the results
    let mut passed = 0;
    let mut f = std::io::stderr();
    for (test, result) in expected.iter() {
        if *result != actual[test] {
            f.write(format!("FAIL {}: {:?} != {:?}\n", test, result, actual[test]).as_bytes());
        }
        else {
            passed += 1;
            // f.write(format!("PASS {}: {:?}\n", test, actual[test]).as_bytes());
        }
    }

    assert_eq!(passed, expected.len());
}

