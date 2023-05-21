use std::env;
use std::process;
use dotenv::dotenv;
use mini_grep::Config;


fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let configs:Config= Config::new(&args).unwrap_or_else(|error|{
        eprintln!("Problem in parsing arguements: {}",error);
        process::exit(1);
    });
    eprintln!("Searching for {}",configs.query);
    eprintln!("Searching for {}",configs.filename);

    if let Err(e) = mini_grep::run(configs) {
        eprintln!("Application Error: {:?}",e);
        process::exit(1);
    }

}
