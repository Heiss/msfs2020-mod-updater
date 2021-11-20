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

  pub fn default_page(mut self, page: &str) -> Self {
    self.default_page = page.into();
    self
  }
}

#[cfg(test)]
mod tests {
  use crate::helpers::settings::Settings;

  #[test]
  fn test_creation() {
    let expected_page = "about";
    let setting = Settings::new("/").default_page(expected_page);

    assert_eq!(setting.default_page, expected_page);
  }
}
