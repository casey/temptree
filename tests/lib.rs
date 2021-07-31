use std::fs;

use tempfile::tempdir;

use temptree::{temptree, tree};

#[test]
fn file() {
  let tmpdir = temptree! {
    foo: "bar",
  };

  let contents = fs::read_to_string(tmpdir.path().join("foo")).unwrap();

  assert_eq!(contents, "bar");
}

#[test]
fn string_filename() {
  let tmpdir = temptree! {
    "foo.txt": "bar",
  };

  let contents = fs::read_to_string(tmpdir.path().join("foo.txt")).unwrap();

  assert_eq!(contents, "bar");
}

#[test]
fn dir() {
  let tmpdir = temptree! {
    foo: {
      bar: "baz",
    },
  };

  let contents = fs::read_to_string(tmpdir.path().join("foo/bar")).unwrap();

  assert_eq!(contents, "baz");
}

#[test]
fn multiple_entries() {
  let tmpdir = temptree! {
    a: "foo",
    b: "bar",
  };

  assert_eq!(fs::read_to_string(tmpdir.path().join("a")).unwrap(), "foo");
  assert_eq!(fs::read_to_string(tmpdir.path().join("b")).unwrap(), "bar");
}

#[test]
fn empty_tree() {
  let tmpdir = temptree! {
    a: {},
  };

  assert_eq!(fs::read_dir(tmpdir.path().join("a")).unwrap().count(), 0);
}

#[test]
fn trailing_comma_optional() {
  temptree! {
    a: {}
  };
}

#[test]
fn map_contents() {
  let mut tree = tree! {
    a: "foo",
    b: "bar",
  };

  tree.map(|_name, contents| contents.to_uppercase());

  let dir = tempdir().unwrap();

  tree.instantiate(dir.path()).unwrap();

  assert_eq!(fs::read_to_string(dir.path().join("a")).unwrap(), "FOO");
  assert_eq!(fs::read_to_string(dir.path().join("b")).unwrap(), "BAR");
}
