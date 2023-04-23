use pronunciation::pronunciation::Pronunciation;


fn main() {
    let pronunciation = Pronunciation::new("cmudict-0.7b_baseform");

    let word = "valo";

    println!("{}", pronunciation.get_kana(word.to_string()));
}
