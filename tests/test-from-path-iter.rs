extern crate dotenv;
extern crate tempdir;

mod common;

use std::env;
use dotenv::*;

use common::*;

#[test]
fn test_from_path() {
    let dir = make_test_dotenv().unwrap();

    let mut path = env::current_dir().unwrap();
    path.push(".env");

    let iter = from_path_iter(&path).unwrap();

    assert!(env::var("TESTKEY").is_err());

    iter.load().ok();

    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
