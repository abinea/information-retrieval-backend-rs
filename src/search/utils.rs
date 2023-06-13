use jieba_rs::Jieba;
use regex::Regex;
use std::{fs, path::Path};

/// 判断是不是中文
pub fn is_chinese(word: &str) {
  let re = Regex::new(r"^[\u4e00-\u9fa5]+$").unwrap();
  assert!(re.is_match(word));
}

pub fn read_file() -> Vec<String> {
  let mut docs = vec![];
  let path = Path::new("./src/pages/");
  // 读取pages目录下的txt文件
  let pages = fs::read_dir(path).unwrap();

  pages.for_each(|res| {
    let path = res.unwrap().path();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_content = fs::read_to_string(&path).unwrap();
    docs.push(file_content);
  });

  docs
}

pub fn cut_word() {
  let jieba: Jieba = Jieba::new();
  let words = jieba.cut("我们中出了一个叛徒", false);
  assert_eq!(words, vec!["我们", "中", "出", "了", "一个", "叛徒"])
}

#[cfg(test)]
mod test {
  #[test]
  fn test_cut_word() {
    super::cut_word();
  }
  #[test]
  fn test_is_chinese() {
    super::is_chinese("我们中出了一个叛徒");
  }
}
