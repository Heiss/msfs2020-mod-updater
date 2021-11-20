use crate::helpers::helper::{check_sign, pub_key};
use crate::repositories::repository::Repository;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repositories {
  sign: Option<String>,
  created_at: Option<String>,
  providers: Vec<Repository>,
}

impl Repositories {
  pub fn new() -> Self {
    Repositories {
      sign: None,
      created_at: None,
      providers: Vec::new(),
    }
  }

  pub fn read(content: &str) -> Self {
    if let Ok(repo) = serde_yaml::from_str::<Repositories>(content) {
      if repo.valid(content) {
        return repo;
      }
    }

    Self::new()
  }

  pub fn valid(&self, content: &str) -> bool {
    if let Some(sign) = &self.sign {
      check_sign(content, pub_key, sign)
    } else {
      true
    }
  }
}
