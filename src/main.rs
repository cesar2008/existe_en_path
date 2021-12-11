use std::env;
use std::process;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_exec_name2() -> String {
    let exe_name= std::env::current_exe()
    .expect("No puedo obtener la ruta del ejecutable")
    .file_name()
    .expect("No puedo obtener la ruta del ejecutable")
    .to_string_lossy()
    .into_owned();
    exe_name
}
fn help() {
    println!("{} \t version {}", get_exec_name2(), VERSION);
    println!("uso:
    {} <RUTA>
    -Checkea que la RUTA que se pasa como parametro se encuentre en la variable de entorno PATH.
    -Devuelve en ERRORLEVEL la cantidad de ocurrencias. ERRORLEVEL==0 si no existe.
    -Si no se ingresa ruta devuelve ERRORLEVEL==-1 y lista todas las rutas cargadas en PATH ordenadas alfabeticamente.",get_exec_name2());
    

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
    }

    let mipath = &args[1];

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

    process::exit(r);
    
 }