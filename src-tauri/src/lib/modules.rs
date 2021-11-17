use std::path::Path;

pub struct CommunityModules {
  list: Vec<Github>,
}

pub impl CommunityModules {
  pub fn new() -> Self {
    CommunityModules { list: Vec::new() }
  }

  pub fn add_mod(self, module: Github) -> Self {
    self.list.push(module);
    self
  }
}

struct Github {}
