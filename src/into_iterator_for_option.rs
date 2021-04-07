#[cfg(test)]
mod test {
    #[test]
    fn implementing_into_iterator_allows_extend() {
        let turing = Some("Turing");
        let mut logicians = vec!["Curry", "Kleene", "Markov"];
        assert_eq!(logicians.len(), 3);
        logicians.extend(turing);
        assert_eq!(logicians.len(), 4);
    }

    #[test]
    fn options_allows_chain() {
        let turing = Some("Turing");
        let logicians = vec!["Curry", "Kleene", "Markov"];
        assert_eq!(logicians.len(), 3);
        let x = logicians.iter().chain(turing.iter());
        assert_eq!(x.into_iter().collect::<Vec<_>>().len(), 4);
    }
}
