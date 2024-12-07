use crate::utils::{CompletionResult, Utils};

#[test]
pub fn test_autocomplete_found_similar() {
    let words: Vec<String> = vec![
        String::from("apple"),
        String::from("orange"),
        String::from("watermelon"),
    ];

    let result = Utils::suggest_completion("ap", words.clone());
    assert_eq!(
        result,
        CompletionResult::FoundSimilar(String::from("apple"))
    )
}

#[test]
pub fn test_autocomplete_found() {
    let words: Vec<String> = vec![
        String::from("apple"),
        String::from("orange"),
        String::from("watermelon"),
    ];

    let result = Utils::suggest_completion("apple", words.clone());
    assert_eq!(result, CompletionResult::Found)
}

#[test]
pub fn test_autocomplete_nothing() {
    let words: Vec<String> = vec![
        String::from("apple"),
        String::from("orange"),
        String::from("watermelon"),
    ];

    let result = Utils::suggest_completion("enjo", words.clone());
    assert_eq!(result, CompletionResult::Nothing)
}
