use reqwest::blocking;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let client = blocking::Client::new();

    for item in &args[1..] {
        println!("SEARCH RESULTS FOR \"{item}\"");
        let r = client.get(format!("https://nvim.sh/s/{item}")).send().expect("Unable to connect");

        let text = r.text().expect("Weird response");
        match text.len() {
            0 => { println!("!! NO SEARCH RESULTS FOR {item}") },
            _ => { println!("{text}") }
        }
        println!();
    };
}
