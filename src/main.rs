extern crate wikipedia;

use std::env::args;
use std::io::stdin;

fn pick_a_number() -> usize {
    let mut selection = String::new();
    println!("Please enter the number of the result you want to translate.");
    stdin().read_line(&mut selection).unwrap();
    selection.trim().parse().unwrap()
}

fn main() {

    let lang = args().nth(1).map(String::from);
    let term = args().nth(2).map(String::from);


    if let (Some(lang), Some(term)) = (lang, term) {

    let mut wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    wiki.language = lang;

    let results = wiki.search(&term).unwrap();
    for (i, result ) in results.iter().enumerate() {
        println!("{}) {}",  i, result);
    }

    if let Some(title) = results.get(pick_a_number()) {
        let page = wiki.page_from_title(title.to_owned());
        for lang in page.get_langs().unwrap() {
            println!("{} ({})", lang.translation, lang.lang);
        }
    }

    }
}
