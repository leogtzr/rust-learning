fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    chars.next().map(|ch| (ch, chars.as_str()))
}

fn is_vowel(first_char: char) -> bool {
    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false
    }
}

fn to_pig_latin(word: String) -> String {
    let mut piglatin = String::new();

    if let Some((first, rest)) = split_first_char(word.as_str()) {
        if is_vowel(first) {
            piglatin = format!("{}-hay", word);
        } else {
            piglatin = format!("{}-{}ay", rest, first);
        }
    } 

    piglatin
}

fn main() {
   let str = String::from("first"); 
   println!("{}", to_pig_latin(str));
}
