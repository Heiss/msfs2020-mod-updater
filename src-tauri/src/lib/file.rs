use ring::digest::{Context, Digest, SHA256};
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct File {
  filepath: PathBuf,
  digest: Option<Digest>,
}

pub impl File {
  pub fn new(path: &str) -> Self {
    File {
      filepath: PathBuf::from(path),
      digest: None,
    }
  }

  pub fn set_digest(mut self, digest: Option<Digest>) -> Self {
    self.digest = digest;
    self
  }

  pub fn check_digest(&self) -> Bool {
    self.digest == self.calc_new_digest()
  }

  fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
      let count = reader.read(&mut buffer)?;
      if count == 0 {
        break;
      }
      context.update(&buffer[..count]);
    }

    Ok(context.finish())
  }

  pub fn calc_digest(&mut self) -> Result<String> {
    self.digest = self.calc_new_digest();
    Ok(HEXUPPER.encode(self.digest.unwrap().as_ref()).into())
  }

  fn calc_new_digest(&self) -> Digest {
    let input = File::open(self.filepath)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    return digest;
  }
}
