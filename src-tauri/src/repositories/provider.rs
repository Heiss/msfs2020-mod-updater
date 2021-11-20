use crate::repositories::repository::Repository;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
  name: String,
  key: String,
  link: String,
  public_key: Option<String>,
  notice: Option<String>,
  repository: Option<Repository>,
}

impl Provider {
  pub fn new(name: &str, key: &str, link: &str) -> Self {
    Provider {
      name: name.to_string(),
      key: key.to_string(),
      link: link.to_string(),
      public_key: None,
      notice: None,
      repository: None,
    }
  }

  pub fn notice(mut self, notice: &str) -> Self {
    self.notice = Some(notice.to_string());
    self
  }

  pub fn set_public_key(mut self, key: &str) -> Self {
    self.public_key = Some(key.to_string());
    self
  }

  pub fn read(content: &str) -> Self {
    serde_yaml::from_str(content).unwrap()
  }

  pub fn load_repository(&mut self) -> &mut Self {
    let content = reqwest::blocking::get(&self.link).unwrap().text().unwrap();
    let repository: Repository = serde_yaml::from_str(&content).unwrap();

    if let Some(key) = &self.public_key {
      if repository.valid(&content, key) {
        self.repository = Some(repository);
      }
    }

    self
  }
}
