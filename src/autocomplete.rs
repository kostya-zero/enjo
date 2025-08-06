use crate::{config::Config, terminal::ask_dialog};

#[derive(Debug, Eq, PartialEq)]
pub enum CompletionResult {
    Found,
    FoundSimilar(String),
    Nothing,
}

pub fn autocomplete(word: &str, words_list: Vec<&str>, config: &Config) -> Option<String> {
    let suggested = suggest_completion(word, words_list.clone());

    match suggested {
        CompletionResult::Found => Some(word.to_string()),
        CompletionResult::FoundSimilar(name) => {
            if config.autocomplete.always_accept {
                return Some(name);
            }
            let answer = ask_dialog(&format!("Did you mean '{name}'?"), true);
            if answer { Some(name) } else { None }
        }
        CompletionResult::Nothing => None,
    }
}

pub fn suggest_completion(word: &str, words_list: Vec<&str>) -> CompletionResult {
    if words_list.contains(&word) {
        return CompletionResult::Found;
    }

    if let Some(similiar) = words_list.iter().find(|entry| entry.starts_with(word)) {
        CompletionResult::FoundSimilar(similiar.to_string())
    } else {
        CompletionResult::Nothing
    }
}
