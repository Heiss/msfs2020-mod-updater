use std::path::Path;

pub struct CommunityModules {
  list: Vec<Github>,
}

impl CommunityModules {
  pub fn new() -> Self {
    CommunityModules { list: Vec::new() }
  }

  pub fn add_mod(self, module: Github) -> Self {
    self.list.push(module);
    self
  }
}

struct Github {}

#[cfg(test)]
mod tests {
  #[test]
  fn test_creation() {
    let mut set = CommunityModules::new().add_mod(Githb {});
    set.add_mod(Github {})
  }
}
