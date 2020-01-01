use std::{collections::HashMap, fs, path::Path};

pub fn tempdir() -> tempfile::TempDir {
  tempfile::Builder::new()
    .prefix("temptree")
    .tempdir()
    .expect("failed to create temporary directory")
}

pub enum Entry {
  File {
    contents: &'static str,
  },
  Dir {
    entries: HashMap<&'static str, Entry>,
  },
}

impl Entry {
  fn instantiate(self, path: &Path) {
    match self {
      Entry::File { contents } => fs::write(path, contents).expect("Failed to write tempfile"),
      Entry::Dir { entries } => {
        fs::create_dir(path).expect("Failed to create tempdir");

        for (name, entry) in entries {
          entry.instantiate(&path.join(name));
        }
      }
    }
  }

  pub fn instantiate_base(base: &Path, entries: HashMap<&'static str, Entry>) {
    for (name, entry) in entries {
      entry.instantiate(&base.join(name));
    }
  }
}

#[macro_export]
macro_rules! entry {
  {
    {
      $($contents:tt)*
    }
  } => {
    $crate::Entry::Dir{entries: $crate::entries!($($contents)*)}
  };
  {
    $contents:expr
  } => {
    $crate::Entry::File{contents: $contents}
  };
}

#[macro_export]
macro_rules! entries {
  {
  } => {
    std::collections::HashMap::new()
  };
  {
    $($name:tt : $contents:tt,)*
  } => {
    {
      let mut entries: std::collections::HashMap<&'static str, $crate::Entry> = std::collections::HashMap::new();

      $(
        entries.insert($crate::name!($name), $crate::entry!($contents));
      )*

      entries
    }
  }
}

#[macro_export]
macro_rules! name {
  {
    $name:ident
  } => {
    stringify!($name)
  };
  {
    $name:literal
  } => {
    $name
  };
}

#[macro_export]
macro_rules! temptree {
  {
    $($contents:tt)*
  } => {
    {
      let tempdir = $crate::tempdir();

      let entries = $crate::entries!($($contents)*);

      $crate::Entry::instantiate_base(&tempdir.path(), entries);

      tempdir
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn temptree_file() {
    let tmpdir = temptree! {
      foo: "bar",
    };

    let contents = fs::read_to_string(tmpdir.path().join("foo")).unwrap();

    assert_eq!(contents, "bar");
  }

  #[test]
  fn temptree_dir() {
    let tmpdir = temptree! {
      foo: {
        bar: "baz",
      },
    };

    let contents = fs::read_to_string(tmpdir.path().join("foo/bar")).unwrap();

    assert_eq!(contents, "baz");
  }
}
