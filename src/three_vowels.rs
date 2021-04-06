pub fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}

pub fn three_consecutive_vowels(sentence_string: &str) -> bool {
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn word() {
        let dinosaurio = "dinosaurio".to_string();
        let curious = "Curious".to_string();
        let borrowed = "Curious";

        assert_eq!(three_vowels(&dinosaurio), false);
        assert_eq!(three_vowels(&curious), true);
        
        assert_eq!(three_vowels("dinosaurio"), false);
        assert_eq!(three_vowels("curious"), true);

        assert_eq!(three_vowels(borrowed), true);
        assert_eq!(three_vowels(borrowed), true);
    }

    #[test]
    fn sentence() {
        let borrowed = "Once upon a time, there was a friendly curious crab named Ferris";

        assert_eq!(three_consecutive_vowels(borrowed), true);

    }
}