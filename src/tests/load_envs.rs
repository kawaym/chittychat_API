#![allow(unused)]

#[cfg(test)]
use super::*;
use crate::tests::utils as test_utils;
use crate::utils;
use crate::utils::load_envs::EnvFile;
use std::env;

#[test]
fn loads_correctly_from_env(){
    test_utils::create_env_files("development");
    let env: EnvFile = utils::load_envs::load();
    assert_eq!(env.port, 8080);
    assert_eq!(env.app_host, "127.0.0.1");
    assert_eq!(env.database_url, "postgresql://username:passsword@host/database");

    test_utils::delete_all_env_files();
}