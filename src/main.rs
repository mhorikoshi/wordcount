//! `wordcount` crateの説明です。

extern crate regex;
use regex::Regex;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};


/// 文字列を読み込み、単語頻度を更新します。
///
/// # Examples
///
/// ```
/// let mut freqs = HashMap<String, i8>::new();
/// freqs = word_count("aa bb".to_string(), freqs);
/// ```
fn word_count(line: String, mut freqs: HashMap<String, i8>)
  -> HashMap<String, i8> {

  let re = Regex::new(r"\w+").unwrap();

  for m in re.find_iter(&line) {
    let word: String = m.as_str().to_string();

    match freqs.entry(word) {
      Occupied(mut view) => { *view.get_mut() += 1; }
      Vacant(view) => { view.insert(1); }
    }
  }
  freqs
}

fn main() {
  let filename = env::args().nth(1).unwrap(); // .or("処理するファイル名を指定してください");
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(&file);

  let mut freqs: HashMap<String, i8> = HashMap::new();

  for line in reader.lines() {
    freqs = word_count(line.unwrap(), freqs);
  }
  println!("{:?}", freqs);
}

#[test]
fn word_count_works() {

  let freqs: HashMap<String, i8> = HashMap::new();
  let mut exp: HashMap<String, i8> = HashMap::new();
  exp.insert("aa".to_string(), 1);
  exp.insert("bb".to_string(), 2);
  exp.insert("cc".to_string(), 1);

  assert_eq!(word_count("aa bb cc bb".to_string(), freqs), exp);
}

/*
#[test]
fn word_count_fails() {

  let freqs: HashMap<String, i8> = HashMap::new();
  let mut exp: HashMap<String, i8> = HashMap::new();
  exp.insert("aa".to_string(), 1);
  exp.insert("bb".to_string(), 2);

  assert_eq!(word_count("aa  cc dd".to_string(), freqs), exp);
}
*/

#[test]
fn word_count_works2() {

  let freqs: HashMap<String, i8> = HashMap::new();
  let mut exp: HashMap<String, i8> = HashMap::new();
  exp.insert("aa".to_string(), 1);
  exp.insert("cc".to_string(), 1);
  exp.insert("dd".to_string(), 1);

  assert_eq!(word_count("aa  cc dd".to_string(), freqs), exp);
}


/*
extern crate test;
use test::Bencher;

#[bench]
fn bench_word_count(b: &mut Bencher) {
    let freqs: HashMap<String, i8> = HashMap::new();
    b.iter(|| word_count("aa bb cc dd".to_string(), freqs));
}
*/

/*
#[cfg(test)]
mod test {

  use std::collections::HashMap;
  use super::word_count;

  #[test]
  fn word_count_works() {

    let freqs: HashMap<String, i8> = HashMap::new();
    let mut exp: HashMap<String, i8> = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(word_count("aa bb cc bb".to_string(), freqs), exp);
  }

  fn assert_word_count(inp: &str, exp: Vec<&str>) {
    let result = word_count(inp.to_string());
    let expected: Vec<String> = exp.iter().map(|x| x.to_string()).collect();

    assert_eq!(result, expected);
  }

  #[test]
  fn word_split_works2() {
    assert_split("aa bb cc", vec!["aa", "bb", "cc"]);
  }

  #[test]
  fn word_split_works3() {
    assert_split("aa,bb.cc", vec!["aa", "bb", "cc"]);
  }
}
*/
