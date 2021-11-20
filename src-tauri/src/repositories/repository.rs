use crate::helpers::helper::check_sign;
use crate::repositories::module::Module;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
  sign: Option<String>,
  created_at: Option<String>,
  modules: Vec<Module>,
}

impl Repository {
  pub fn new(sign: &str, created_at: &str) -> Self {
    Repository {
      sign: Some(sign.to_string()),
      created_at: Some(created_at.to_string()),
      modules: Vec::new(),
    }
  }

  pub fn default() -> Self {
    Repository {
      sign: None,
      created_at: None,
      modules: Vec::new(),
    }
  }

  fn add_module(&mut self, module: Module) -> &mut Self {
    self.modules.push(module);
    self
  }

  pub fn read(content: &str) -> Self {
    serde_yaml::from_str(content).unwrap()
  }

  pub fn valid(&self, content: &str, pub_key: &str) -> bool {
    if let Some(sign) = &self.sign {
      check_sign(content, pub_key, sign)
    } else {
      true
    }
  }
}
