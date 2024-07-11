extern crate unicode_segmentation;

use std::io;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
  // given a hardcoded word, convert to pig latin (phase 1)
  // given a hardcoded sentence, convert to pig latin (phase 2)
  // given an enter terminated string on stdin, convert to pig latin (phase 3)
  let mut s = String::new();

  println!("Enter a string to Pig-Latinify:");
  io::stdin().read_line(&mut s).expect("Error reading line");

  let words: Vec<&str> = s.as_mut_str().split(' ').collect();

  let mut output_list: Vec<String> = Vec::new();
  for word in &words {
    // split into characters
    let mut chars = UnicodeSegmentation::graphemes(*word,true)
      .collect::<Vec::<&str>>();
    let _ = &output_list.push(piggify(&mut chars));
  }

  let output_str = output_list.join(" ");

  println!("{output_str}");
}

fn piggify(word: &mut Vec<&str>) -> String {
  let first_char: char = word.get(0).expect("Could not unwrap Vec").chars().next().unwrap();

  match first_char {
    'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => return handle_vowel(word),
    _ => return handle_consonant(word)
  };

  fn handle_vowel(word: &mut Vec<&str>) -> String {
    const HAY: &str = "-hay";
    let mut pl_word = word.join("");
    pl_word.push_str(HAY);
    pl_word
  }

  fn handle_consonant(word: &mut Vec<&str>) -> String {
    const AY: &str = "ay";
    let first_letter = word[..1].join("");
    let mut rest = word[1..].join("");
    rest.push('-');
    rest.push_str(&first_letter);
    rest.push_str(AY);
    rest
  }
}