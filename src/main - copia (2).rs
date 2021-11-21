use std::env;

fn main() {
    let nomvar="PATH";
    let u = match env::var_os(nomvar) {
        Some(v) => v.into_string().unwrap(),
        None => panic!( "no seteado {} $nomvar", nomvar )
    };
    //let s="banana";
    println!("{}: {}", nomvar,u);
    println!( "{}",u.to_uppercase().contains("C:\\WINDOWS\\System32") );
    assert_eq!( Some(u.to_uppercase()), Some("C:\WINDOWS\System32") );
    println!("C:\\WINDOWS\\System32");
    //println!(u.find("bin"));
}