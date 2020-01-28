extern crate toml;
extern crate xhtmlchardet;
#[macro_use]
extern crate serde_derive;

use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Deserialize)]
struct Test {
    // src: String,
    charset: Vec<String>,
    variant: String,
    // content_type_header: String,
}

fn read_config() -> Vec<Test> {
    let mut file = File::open("tests/fixtures.toml").expect("Unable to open tests/fixtures.toml");
    let mut toml = String::new();
    file.read_to_string(&mut toml)
        .expect("Error reading config file");
    let mut config: HashMap<String, Vec<Test>> =
        toml::decode_str(&toml).expect("Error parsing config file");
    config
        .remove("fixtures")
        .expect("no fixtures in config file")
}

#[test]
fn test_fixtures() {
    let tests = read_config();

    // Map the filename to the output charset
    let mut expected: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut actual: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for test in tests {
        let path = format!("tests/{}-{}.txt", &test.charset[0], &test.variant);
        expected.insert(path.clone(), test.charset);

        let mut file = File::open(&path).expect(&format!("Unable to open {}", path));
        let actual_charset = xhtmlchardet::detect(&mut file, None);
        actual.insert(path, actual_charset.unwrap());
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
