fn main() {
    test_an_empty_string();
    test_a_word();
    test_a_capitalized_word();
    test_a_sentence_with_punctuation();
    test_a_palindrome();
    test_an_even_sized_word();
    test_wide_characters();
    test_grapheme_clusters();
}

fn reverse(input: &str) -> String {
    
    let mut my_chars: Vec<char> = input.chars().collect();
    my_chars.reverse();

    let s: String = my_chars.iter().collect();
    String::from(s)
}


fn process_reverse_case(input: &str, expected: &str) {
    let given = reverse(input);
    println!("Input: {} --- Expected: {} --- Given: {}",input,expected,given);
   
}

/// empty string
fn test_an_empty_string() {
    process_reverse_case("", "");
}

/// a word
fn test_a_word() {
    process_reverse_case("robot", "tobor");
}

/// a capitalized word
fn test_a_capitalized_word() {
    process_reverse_case("Ramen", "nemaR");
}

/// a sentence with punctuation
fn test_a_sentence_with_punctuation() {
    process_reverse_case("I'm hungry!", "!yrgnuh m'I");
}

/// a palindrome
fn test_a_palindrome() {
    process_reverse_case("racecar", "racecar");
}

/// an even-sized word
fn test_an_even_sized_word() {
    process_reverse_case("drawer", "reward");
}

/// wide characters
fn test_wide_characters() {
    process_reverse_case("子猫", "猫子");
}


/// grapheme clusters
fn test_grapheme_clusters() {
    process_reverse_case("uüu", "uüu");
}