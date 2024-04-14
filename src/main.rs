use std::env;
use std::process;
use minigrep::run;
use minigrep::Config;

fn main() {
    let args:Vec<String> = env::args().collect();
    //let query = &args[1];
    //let filename = &args[2];
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("{}", err);
        process::exit(1)

    });

    // eprintln!("/** {:?} /**", config);
 

    if let Err(e) = run(config){
        eprintln!("Application error: {}", e);
        process::exit(1)
    }

    
}
// cross build --target x86_64-pc-windows-gnu --release


