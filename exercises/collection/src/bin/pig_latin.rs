extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
  // given a hardcoded word, convert to pig latin (phase 1)
  // given a hardcoded sentence, convert to pig latin (phase 2)
  // given an enter terminated string on stdin, convert to pig latin (phase 3)

  // phase 1
  let word = String::from("irst");

  // split into characters
  let mut chars = UnicodeSegmentation::graphemes(word.as_str(),true)
    .collect::<Vec::<&str>>();
  // examine the first character (is it a consonant or vowel)
  println!("pig latin word: {}", piggify(&mut chars));
    // consonant
      // remove the first character
      // create suffix string (-fay)
      // append sufix to remaining partial string
      // return word
    // vowel
      // append vowel suffix (-hay) to word
      // return word


}

fn piggify(word: &mut Vec<&str>) -> String {
  let first_char: char = word.get(0).unwrap().chars().next().unwrap();
  
  fn handle_vowel(word: &mut Vec<&str>) -> String {
    const HAY: &str = "-hay";
    let mut pl_word = word.join("");
    pl_word.push_str(HAY);
    pl_word
  }

  match first_char {
    'a'|'e'|'i'|'o'|'u' => handle_vowel(word),
    'A'|'E'|'I'|'O'|'U' => handle_vowel(word),
    other => {
      println!("the case of {}", other);
      other.to_string()
    }
  }
}