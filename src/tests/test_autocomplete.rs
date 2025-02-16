use crate::utils::{CompletionResult, Utils};

#[test]
pub fn test_autocomplete_found_similar() {
    let words: Vec<&str> = vec!["apple", "orange", "watermelon"];

    let result = Utils::suggest_completion("ap", words);
    assert_eq!(
        result,
        CompletionResult::FoundSimilar(String::from("apple"))
    )
}

#[test]
pub fn test_autocomplete_found() {
    let words: Vec<&str> = vec!["apple", "orange", "watermelon"];

    let result = Utils::suggest_completion("apple", words);
    assert_eq!(result, CompletionResult::Found)
}

#[test]
pub fn test_autocomplete_nothing() {
    let words: Vec<&str> = vec!["apple", "orange", "watermelon"];

    let result = Utils::suggest_completion("enjo", words);
    assert_eq!(result, CompletionResult::Nothing)
}
