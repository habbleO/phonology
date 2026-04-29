pub struct Phoneme {
    symbol: String
}

impl Phoneme {
    pub fn get_symbol(&self) -> &String {
        return &self.symbol;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let phone = String::from("a");
        let phoneme = Phoneme{symbol: phone};
        let get_phone = phoneme.get_symbol();
        assert_eq!(get_phone, &String::from("a"));
    }
}
