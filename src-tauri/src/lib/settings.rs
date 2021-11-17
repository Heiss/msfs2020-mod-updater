use std::path::Path;

pub struct Settings {
  community_folder: Path,
  default_page: String,
}

pub impl Settings {
  pub fn new(path: &str) -> Self {
    Settings {
      community_folder: Path::new(path),
      default_page: "".into(),
    }
  }

  pub fn default_page(self, page: &str) -> Self {
    self.default_page = page.into();
    self
  }
}
