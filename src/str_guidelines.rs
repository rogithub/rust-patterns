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

pub fn say_hello(name: &str) -> String {
    // We could construct the result string manually.
    // let mut result = "Hello ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result

    // But using format! is better.
    format!("Hello {}!", name)
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

    #[test]
    fn saludo() {
        let borrowed = "Ferris";

        assert_eq!(say_hello(borrowed), "Hello Ferris!");        

    }
}