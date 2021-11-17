use std::path::Path;

fn find_community_folder() -> String {
  todo!();
}

fn unzip(path: Path, dist: &Path) {
  todo!()
}

fn unrar(path: Path, dist: &Path) {
  todo!()
}

fn unpack(path: &Path, dist: &Path) {
  if path.ends_with(".zip") {
    unzip(path, dist)
  }

  if path.ends_with(".rar") {
    unrar(path, dist)
  }
}
