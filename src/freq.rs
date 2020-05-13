// a module to analyze letter frequencies


fn expected(c: char) -> f32 {
  match c {
    'e' => 0.1202,
    't' => 0.091,
    'a' => 0.0812,
    'o' => 0.0768,
    'i' => 0.0731,
    'n' => 0.0695,
    's' => 0.0628,
    'r' => 0.0602,
    'h' => 0.0592,
    'd' => 0.0432,
    'l' => 0.0398,
    'u' => 0.0288,
    'c' => 0.0271,
    'm' => 0.0261,
    'f' => 0.0230,
    'y' => 0.0211,
    'w' => 0.0209,
    'g' => 0.0203,
    'p' => 0.0182,
    'b' => 0.0149,
    'v' => 0.0111,
    'k' => 0.0069,
    'x' => 0.0017,
    'q' => 0.0011,
    'j' => 0.001,
    'z' => 0.0007,
    ' ' => 0.0,
    _ => -0.1
  }
}

pub fn score(text: &Vec<char>) -> f32 {
  let downcased: Vec<char> = text.iter().map(|n| n.to_ascii_lowercase()).collect();
  let mut score = 0f32;
  for chunk in &downcased {
    score += expected(*chunk);
  };
  score / downcased.len() as f32
}

#[test]
fn letter_frequency() {
  assert_eq!(expected('e'), 0.1202)
}

#[test]
fn scoring() {
  assert_eq!(score(&vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd']), 0.052427277);
  assert_eq!(score(&vec!['H', 'E', 'L', 'L', 'O', ' ', 'W', 'O', 'R', 'L', 'D']), 0.052427277);
}