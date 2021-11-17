use std::path::PathBuf;

pub struct Settings {
  community_folder: PathBuf,
  default_page: String,
}

impl Settings {
  pub fn new(path: &str) -> Self {
    Settings {
      community_folder: PathBuf::from(path),
      default_page: "".into(),
    }
  }

  pub fn default_page(self, page: &str) -> Self {
    self.default_page = page.into();
    self
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_creation() {
    let mut set = Settings::new().default_page("");
    set.default_page("about")
  }
}
