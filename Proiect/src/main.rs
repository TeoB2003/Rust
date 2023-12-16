use std::env;

fn main() {
    let prog_n = env::args().next();
    let fisiere: Vec<String> = env::args().collect();
    let mut locatii: Vec<String>=Vec::new();
    // Print the program name
    if let Some(name) = prog_n {
        println!("Program Name: {}", name);
    } else {
        println!("Unable to determine the program name");
    }
    let mut nrloc=0;
    // Print each command-line argument
    for (index, arg) in fisiere.iter().skip(1).enumerate() 
    {
        let path;
        if let Some(colon_index) = arg.find(':') {
            let f_type = &arg[..colon_index];
            path = &arg[colon_index + 1..];
            if f_type!="ftp" && f_type != "zip" && f_type!="folder"
            {
                panic!("Ați introdus un tip de locație invalid");
            }
        } else {
            panic!("Formatul este: tip_fisier: path");
        }
        println!("Calea este {}", path);
        locatii.push(path.to_string());
        nrloc=index;
    }println!("Nr locatii= {}", nrloc);
}