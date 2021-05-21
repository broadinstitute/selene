fn main() {
    match selene::run() {
        Ok(_) => { println!("Done!") }
        Err(error) => {
            if error.is_clap_pseudo_error() {
                println!()
            } else {
                println!("Error: {}", error)
            }
        }
    }
}