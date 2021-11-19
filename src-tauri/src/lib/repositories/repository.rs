pub struct Repository {
  name: String,
  key: String,
  url: String,
  public_key: Option<String>,
  notice: Option<String>,
}

pub struct Extra {}

impl Repository {
  pub fn new(name: &str, key: &str, url: &str) -> Self {
    Repository {
      name,
      key,
      url,
      public_key: None,
      notice: None,
    }
  }

  pub fn notice(self, notice: &str) -> Self {
    self.notice = Some(notice);
    self
  }

  pub fn public_key(self, key: &str) -> Self {
    self.public_key = Some(key);
    self
  }
}
