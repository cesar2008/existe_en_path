use std::env;

fn main() {
    // Ejemplos de:
    // cadena larga en el editor de codigo
    // Argumentos de linea de comando
    // Variables de entorno
    //   
    let cadena = "esto es un ejemplo \
    de un texto largo \
    en el editor de código";
    println!("{}",cadena);

    let args: Vec<String> = env::args().collect();
    println!("\n--- PARAMETROS DE ENTRADA -----");
    println!("{:?}", args);
    println!("--- FIN PARAMETROS DE ENTRADA -----");

    println!("\n--- VARIABLES DE ENTORNO -----");
    for (n,v) in env::vars() {
        println!("{}: {}", n,v);
    }
    println!("--- FIN VARIABLES DE ENTORNO -----");

    println!("\n--- LA VARIABLE PATH -----");
    let nomvar="PATH";
    let u = match env::var_os(nomvar) {
        Some(v) => v.into_string().unwrap(),
        None => panic!( "no seteado {} $nomvar", nomvar )
    };
   //let s="banana";
    println!("{}: {}", nomvar,u);
    println!( "esta USR en PATH ? {}",u.to_uppercase().contains("USR") );
    //println!(u.find("bin"));
   println!("--- FIN LA VARIABLE PATH -----");
 }