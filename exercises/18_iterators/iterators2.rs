// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use std::{vec};

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut x: String = match chars.next() {
        None => String::new(),
        Some(first) => first.to_string().to_uppercase(),
    };
    if x.is_empty(){
        x
    }
    else {
        for i in chars{
            x.push(i);
        }
         x
    }


}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut x: Vec<String> = vec![];
    for i in words.iter(){
        x.push(capitalize_first(i));
    }
    x
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut y: String = Default::default();
    let x = capitalize_words_vector(words);
    for i in x.iter(){
        y.push_str(i);
    }
     y
}

fn main() {
    // You can optionally experiment here.
    let x = "hahah";
    print!("{}",capitalize_first(x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
