use env::APIKeys;

mod env;

fn main() {
    let creds = APIKeys::default();
    println!("Creds: {:#?}", creds);
}
