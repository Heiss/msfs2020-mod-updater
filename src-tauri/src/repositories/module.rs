use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
  name: String,
  key: String,
  version: String,
  sign: Option<String>,
  download_url: String,
  notice: Option<String>,
  extras: Vec<Extra>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extra {}

impl Module {
  pub fn new(name: &str, key: &str, download_url: &str, version: &str) -> Self {
    Module {
      name: name.to_string(),
      key: key.to_string(),
      download_url: download_url.to_string(),
      sign: None,
      extras: Vec::new(),
      notice: None,
      version: version.to_string(),
    }
  }

  pub fn notice(mut self, notice: &str) -> Self {
    self.notice = Some(notice.to_string());
    self
  }

  pub fn sign(mut self, sign: &str) -> Self {
    self.sign = Some(sign.to_string());
    self
  }

  pub fn add_extras(&mut self, extra: Extra) -> &mut Self {
    self.extras.push(extra);
    self
  }

  pub fn read(content: &str) -> Self {
    serde_yaml::from_str(content).unwrap()
  }
}
