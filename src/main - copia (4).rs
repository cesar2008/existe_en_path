use std::env;
use std::path::Path;

fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}
fn get_exec_name2() -> String {
    std::env::current_exe()
    .expect("Can't get the exec path")
    .file_name()
    .expect("Can't get the exec name")
    .to_string_lossy()
    .into_owned()
}
fn help() {

    let args1 = env::args().next().unwrap();
    println!("args1= {}",args1);
    let path = Path::new( &args1 );    
    println!("uso:
    {} <RUTA>
    Checkea que la RUTA que se pasa como parametro se encuentre en la variable de entorno PATH.
    Devuelve ERRORLEVEL==0 en caso de que exista.",get_exec_name2());
    println!("{:?}",Path::new(&env::args().next().unwrap()).file_name());
    println!("{}", Path::new(&env::args().next().unwrap()).file_name().unwrap().to_str().unwrap() );
}

fn main() {
    // Ejemplos de:
    // cadena larga en el editor de codigo
    // Argumentos de linea de comando
    // Variables de entorno
    //   
    //let cadena = "esto es un ejemplo \
    //de un texto largo \
    //en el editor de código";
    //println!("{}",cadena);

    let args: Vec<String> = env::args().collect();
    println!("largo {}", args.len());
    if args.len() < 2 {
        help();
        return;
    }

    let mipath = &args[1];
    println!( "mipath= {}", mipath );

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