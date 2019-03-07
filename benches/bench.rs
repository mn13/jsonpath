#![feature(test)]
extern crate jsonpath_lib as jsonpath;
extern crate serde_json;
extern crate test;

use std::io::Read;
use serde_json::Value;
use self::test::Bencher;

fn read_json(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

#[bench]
fn bench_reader(b: &mut Bencher) {
    let string = read_json("./benches/example.json");
    let json: Value = serde_json::from_str(string.as_str()).unwrap();
    let mut reader = jsonpath::reader(json);
    b.iter(move || {
        for _ in 1..10000 {
            let _ = reader("$.store").unwrap();
        }
    });
}