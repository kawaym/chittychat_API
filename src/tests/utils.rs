#![allow(unused)]

use std::fs::{self, File};
use std::io::prelude::*;


pub fn delete_all_env_files() {
//! Deletes the env files created for the tests
    let dir_entries = fs::read_dir("./").expect("Test failed reading the env files");
    for entry in dir_entries {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "env") {
            fs::remove_file(path).expect("Test failed deleting the env files");
        }
    }
}

pub fn create_env_files(env_type: &str) {
//! Creates the files neeeded by the tests. Normally this file is manually created by the project owner
    let env_template = b"PORT=8080
DATABASE_URL=postgresql://username:passsword@host/database
APP_HOST=127.0.0.1";

    let mut file: File = File::create(format!(".{}.env", env_type)).expect("Test failed creating needed files");
    file.write_all(env_template).expect("Test failed writing needed info");
}

pub fn create_empty_file(){
    File::create(".test.env").expect("Test failed creating empty file");
}