use modules::Module;

pub struct Repository {
  name: String,
  key: String,
  url: String,
  public_key: Option<String>,
  notice: Option<String>,
  modules: Vec<Module>,
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
      modules: Vec::new(),
    }
  }

  pub fn notice(mut self, notice: &str) -> Self {
    self.notice = Some(notice);
    self
  }

  pub fn public_key(mut self, key: &str) -> Self {
    self.public_key = Some(key);
    self
  }

  fn add_module(&mut self, module: Module) -> &mut Self {
    self.modules.append(module);
    self
  }
}
