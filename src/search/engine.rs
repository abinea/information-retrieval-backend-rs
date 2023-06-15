use super::utils::{cur_for_search, is_chinese, read_files, stem_english_words};
use std::{collections::HashMap, vec};

struct DocsPair(usize, usize);

pub struct Engine {
  indexMap: HashMap<&'static str, usize>,
  docs: HashMap<usize, String>,
  wordSet: Vec<String>,
  terms: Vec<String>,
  cosineSimilarityMap: HashMap<DocsPair, f64>,
}

impl Engine {
  pub fn new() -> Self {
    Self {
      indexMap: HashMap::new(),
      docs: read_files(),
      wordSet: vec![],
      terms: vec![],
      cosineSimilarityMap: HashMap::new(),
    }
  }

  pub fn load(&self) {}

  fn buildIndexedMap(&self) {
    let mut posting_list = HashMap::new();
    let mut terms = HashMap::new();
    let mut word_set = HashMap::new();

    for (i, doc) in self.docs.iter() {
      let words = cur_for_search(doc);
      let mut vec = vec![];
      for word in words {
        if is_chinese(word) {
          word_set.insert(word, true);

          if posting_list.contains_key(word) {
            posting_list
              .entry(word)
              .and_modify(|v: &mut Vec<&usize>| v.push(i));
          } else {
            posting_list.insert(word, vec![i]);
          }

          vec.push(word);
        }
      }

      for word in stem_english_words(doc) {
        let word = word.as_ref();
        word_set.insert(word, true);

        if posting_list.contains_key(word) {
          posting_list
            .entry(word)
            .and_modify(|v: &mut Vec<&usize>| v.push(i));
        } else {
          posting_list.insert(word, vec![i]);
        }

        vec.push(word);
      }
      terms.insert(i, vec);
    }
  }

  pub fn search(&self, query: &str) {}
}
