use ring::hmac::{verify, Key, HMAC_SHA256};
use std::path::PathBuf;

pub fn find_community_folder() -> String {
  todo!();
}

fn unzip(_path: &PathBuf, dist: &PathBuf) {
  todo!()
}

fn unrar(_path: &PathBuf, dist: &PathBuf) {
  todo!()
}

pub fn unpack(path: &PathBuf, dist: &PathBuf) {
  if path.ends_with(".zip") {
    unzip(path, dist)
  }

  if path.ends_with(".rar") {
    unrar(path, dist)
  }
}

pub fn check_sign(content: &str, key: &str, expected_value: &str) -> bool {
  let key = Key::new(HMAC_SHA256, key.as_bytes());
  verify(&key, content.as_bytes(), expected_value.as_bytes()).is_ok()
}

pub const pub_key: &str = "ABC";
