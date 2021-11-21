use std::env;
//use std::path::Path;
use std::process;

fn get_exec_name2() -> String {
    std::env::current_exe()
    .expect("Can't get the exec path")
    .file_name()
    .expect("Can't get the exec name")
    .to_string_lossy()
    .into_owned()
}
fn help() {
    println!("uso:
    {} <RUTA>
    Checkea que la RUTA que se pasa como parametro se encuentre en la variable de entorno PATH.
    Devuelve en ERRORLEVEL la cantidad de ocurrencias.",get_exec_name2());
    
    //println!("\n\n--- LA VARIABLE PATH ---");
    /*let nomvar="PATH";
    let u = match env::var_os(nomvar) {
        Some(v) => {
            println!("\n{:?}",v.into_string().unwrap());
        },
        None => panic!( "no seteado {} $nomvar", nomvar )
    };*/
    //println!("{}={}", nomvar,u);
    let mut r=0;
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            println!("\n\nPATH ordenado:\n---");
            let mut vpath = env::split_paths(&paths).collect::<Vec<_>>();
            vpath.sort();
            for path in vpath{
                r = r + 1;
                println!("{}",path.display());
            }
            /*for path in env::split_paths(&paths) {
                println!("{} '{}'", r, path.display());
            }*/
            println!("---\nse listaron {} rutas",r)
        }
        None => println!("La variable de entorno {} no esta definida.", key)
    }    
    //println!("--- FIN LA VARIABLE PATH ---");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        process::exit(-1);
        //return
    }

    let mipath = &args[1];
    //println!( "mipath= {}", mipath );

    let mut r=0;
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                //let b: bool = path.to_str().unwrap().to_uppercase().contains( &(mipath.to_uppercase()) );
                let b: bool = path.to_str().unwrap().to_uppercase().eq( &(mipath.to_uppercase()) );
                if b {
                    r = r + 1;
                    println!("{} '{}'", r, path.display());
                }
            }
        }
        None => println!("La variable de entorno {} no esta definida.", key)
    }
//    let r: bool = u.to_uppercase().contains( &(mipath.to_uppercase()) );
//    println!( "esta {} en PATH ? {}",mipath, r );

    process::exit(r);
    
 }