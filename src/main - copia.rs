use std::env;

fn run_app() -> Result<(), ()> {
    // Application logic here
    let name = "PATH";
    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v ),
        Err(e) => panic!("${} is not set ({})", name, e)
    }

    println!("{}: {}", nomvar,v);
    println!( "{}",v.to_uppercase().contains("SQL") );

    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}