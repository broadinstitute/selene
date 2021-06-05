fn main() {
    match selene::run() {
        Ok(_) => {
            println!("Done!");
            std::process::exit(0)
        }
        Err(error) => {
            if error.is_clap_pseudo_error() {
                println!();
                std::process::exit(0)
            } else {
                println!("Error: {}", error);
                std::process::exit(error.error_code())
            }
        }
    }
}