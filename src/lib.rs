//! Temptree creates temporary trees of files:
//!
//! ```
//! use temptree::temptree;
//!
//! let tree = temptree! {
//!   foo: "a",
//!   bar: {
//!     baz: "b",
//!   },
//! };
//!
//! let foo_contents = std::fs::read_to_string(tree.path().join("foo")).unwrap();
//! assert_eq!(foo_contents, "a");
//!
//! let baz_contents = std::fs::read_to_string(tree.path().join("bar/baz")).unwrap();
//! assert_eq!(baz_contents, "b");
//! ```

use std::{collections::BTreeMap, fs, io, path::Path};

#[derive(Default)]
pub struct Tree {
  entries: BTreeMap<&'static str, Entry>,
}

impl Tree {
  pub fn instantiate(&self, base: &Path) -> io::Result<()> {
    for (name, entry) in &self.entries {
      entry.instantiate(&base.join(name))?;
    }
    Ok(())
  }

  pub fn insert(&mut self, name: &'static str, entry: Entry) {
    self.entries.insert(name, entry);
  }

  pub fn map(&mut self, mut f: impl FnMut(&str, &str) -> String) {
    let mut stack = vec![self];

    while let Some(tree) = stack.pop() {
      for (name, entry) in &mut tree.entries {
        match entry {
          Entry::File { contents } => {
            *contents = f(name, contents);
          }
          Entry::Tree { tree } => stack.push(tree),
        }
      }
    }
  }
}

pub enum Entry {
  File { contents: String },
  Tree { tree: Tree },
}

impl Entry {
  fn instantiate(&self, path: &Path) -> io::Result<()> {
    match self {
      Entry::File { contents } => fs::write(path, contents)?,
      Entry::Tree { tree } => {
        fs::create_dir(path)?;

        for (name, entry) in &tree.entries {
          entry.instantiate(&path.join(name))?;
        }
      }
    }
    Ok(())
  }
}

#[macro_export]
macro_rules! entry {
  {
    {
      $($contents:tt)*
    }
  } => {
    $crate::Entry::Tree{tree: $crate::tree!($($contents)*)}
  };
  {
    $contents:expr
  } => {
    $crate::Entry::File{contents: $contents.into()}
  };
}

#[macro_export]
macro_rules! tree {
  {} => { $crate::Tree::default() };
  { $($name:tt : $contents:tt),* $(,)? } => {
    {
      #[allow(unused_mut)]
      let mut tree = $crate::Tree::default();

      $( tree.insert($crate::name!($name), $crate::entry!($contents)); )*

      tree
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
      $crate::temptree_result!($($contents)*).expect("failed to instantiate temptree")
    }
  }
}

#[macro_export]
macro_rules! temptree_result {
  {
    $($contents:tt)*
  } => {
    {
      tempfile::Builder::new().prefix("temptree").tempdir()
        .and_then(|tempdir| {
          $crate::tree!($($contents)*).instantiate(&tempdir.path())?;
          Ok(tempdir)
        })
    }
  }
}
