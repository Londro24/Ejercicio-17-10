use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

fn read_file(mut f: &File){
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}

fn create_blank_file(p: &Path){
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}


fn add_new_content(mut f: &File){
    f.write_all(b"Nuevo texto\n");
}

fn open_file_to_append(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    
    return file
}

fn open_file_to_read(p: &Path){
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file)
    } else {
        create_blank_file(p);
    }
}

fn main(){
    let path = Path::new("./hola.txt");
    open_file_to_read(path);
    let file = open_file_to_append(path);
    add_new_content(&file);
    open_file_to_read(path);
}