use super::utils::read_file;
use std::collections::HashMap;

struct DocsPair(usize, usize);

pub struct Engine {
  // pub indexMap: HashMap<&'static str, usize>,
  // pub docs: Vec<String>,
  // pub wordSet: Vec<String>,
  // pub terms: Vec<String>,
  // pub cosineSimilarityMap: HashMap<DocsPair, f64>,
}

impl Engine {
  pub fn new() -> Self {
    Self {}
  }

  pub fn search(&self, query: &str) {}
}
