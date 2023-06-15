use jieba_rs::Jieba;
use lazy_static::lazy_static;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use std::{borrow::Cow, collections::HashMap, fs};

lazy_static! {
  // 创建一个英文词干提取器
  static ref en_stemmer: Stemmer = Stemmer::create(Algorithm::English);
  // 中文jieba分词器
  static ref jieba: Jieba = Jieba::new();
  static ref en_re: Regex = Regex::new(r"\b\w+\b").unwrap();
}

/// 判断是不是中文
pub fn is_chinese(word: &str) -> bool {
  let re = Regex::new(r"^[\u4e00-\u9fa5]+$").unwrap();
  re.is_match(word)
}

pub fn read_files() -> HashMap<usize, String> {
  let mut docs: HashMap<usize, String> = HashMap::new();
  // 读取pages目录下的txt文件
  let pages = fs::read_dir("./pages/").unwrap();
  let mut cnt = 0;
  pages.for_each(|res| {
    let path = res.unwrap().path();
    let file_content = fs::read_to_string(&path).unwrap();
    docs.insert(cnt, file_content);
    cnt += 1
  });
  docs
}

pub fn stem_english_words(doc: &str) -> Vec<String> {
  en_re
    .find_iter(doc)
    .map(|mat| en_stemmer.stem(mat.as_str()).to_lowercase())
    .collect::<Vec<String>>()
}

pub fn cur_for_search(doc: &str) -> Vec<&str> {
  jieba.cut_for_search(doc, false)
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_read_files() {
    let docs = read_files();
    assert_eq!(docs.len(), 54);
  }

  #[test]
  fn test_stem_english_words() {
    let doc = "I am a student";
    let words = stem_english_words(doc);
    assert_eq!(words, vec!["i", "am", "a", "student"]);
  }

  #[test]
  fn test_cur_for_search() {
    let doc = "我是一个学生";
    let words = cur_for_search(doc);
    assert_eq!(words, vec!["我", "是", "一个", "学生"]);
  }
}
