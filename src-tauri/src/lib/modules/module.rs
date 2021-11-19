pub struct Module {
  name: String,
  key: String,
  version: String,
  sign: String,
  download_url: String,
  notice: Option<String>,
  extras: Vec<Extra>,
}

pub struct Extra {}

impl Module {
  pub fn new(name: &str, key: &str, download_url: &str, sign: &str) -> Self {
    Module {
      name,
      key,
      download_url,
      sign,
      extras: Vec::new(),
      notice: None,
    }
  }

  pub fn notice(self, notice: &str) -> Self {
    self.notice = Some(notice);
    self
  }

  pub fn add_extras(self, extra: Extra) -> Self {
    self.extras.append(extra);
    self
  }
}
