use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::SystemTime;


fn get_directory_contents(directory_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut entries = Vec::new();

    if let Ok(entries_iter) = fs::read_dir(directory_path) {
        for entry in entries_iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    entries.push(path.display().to_string());
                    if let Ok(subdir_entries) = get_directory_contents(&path.to_string_lossy()) {
                        entries.extend(subdir_entries);
                    }
                } else {
                    entries.push(path.display().to_string());
                }
            }
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Error reading directory: {}", directory_path),
        ));
    }

    Ok(entries)
}

fn initial_f(loc: Vec<(String, String)>) {
    let mut index=0;
    let mut all_entries = HashMap::new();
    let mut f_names: Vec<String>=Vec::new();

    for (_, path) in loc.iter() {
        let last_component = path.rsplit('\\').next();
        match last_component {


            Some(component) => {f_names.push(component.to_string());println!("Locatii {component}")},
            None => eprintln!("Path does not have a last component"),
        }
        match get_directory_contents(path) {
            Ok(entries) => {
                for entry in entries.iter() {
                    let mut nt:SystemTime=SystemTime::now();
                    let  mut ot:SystemTime=SystemTime::now();
                    match fs::metadata(entry)
                    {
                        Ok(metadate)=> nt=metadate.modified().unwrap(),
                        Err(e)=> println!("eroare {}",e),
                    }                      

                    let as1=&entry[entry.find(last_component.unwrap()).unwrap()+last_component.unwrap().to_string().len()..];
                    if all_entries.contains_key(as1)
                    {
                        let b: usize=*all_entries.get(as1).unwrap();
                        let locv: String=loc[b].1.clone()+ as1;
                        println!("locatia este {locv}");
                        match fs::metadata(locv)
                        {
                            Ok(metadate)=> ot=metadate.modified().unwrap(),
                            Err(e)=> println!("eroare {}",e),
                        }  
                        match nt.cmp(&ot) {
                       
                            std::cmp::Ordering::Greater => {
                                all_entries.insert(as1.to_string(),index);
                            }
                            std::cmp::Ordering::Equal => {
                                println!("Next");
                            }
                            std::cmp::Ordering::Less => {
                                println!("Next");
                            }
                        }
                          
                    }
                    else{
                    println!("Locatie={as1}");
                   
                        all_entries.insert(as1.to_string(),index);
                    }
                    
                }
            }
            Err(e) => {
                eprintln!("Error processing directory {}: {}", path, e);
            }
        }
        index+=1;
    }
    index=0;
    for (_,ceva) in loc.iter(){
        println!("C {ceva}");
        let locatii=get_directory_contents(ceva).unwrap();
        for ( b,k) in all_entries.clone()
        {
            let found = locatii.iter().any(|s1| s1.contains(&b));
            if found && k== index{
                println!("Fisierul '{}' exista.", b);
            } else {
               let loc_s=loc[k].1.clone()+&b;
               let loc_d=ceva.clone()+&b; 
               println!("Mut de la {loc_s} la {loc_d}");
               match fs::copy(loc_s, loc_d) {
                Ok(_) => {
                    println!("Fișierul a fost copiat cu succes.");
                }
                Err(e) => {
                    eprintln!("Eroare la copierea fișierului: {}", e);
                }
            }
            }
            index+=1;
        }
    }
 println!("All Entries: {:?}", all_entries);
}



fn main() {
    let fisiere: Vec<String> = env::args().collect();
    let mut locatii: Vec<(String,String)>=Vec::new();
   
    // Print each command-line argument
    for (index, arg) in fisiere.iter().skip(1).enumerate() 
    {
        let f_type;
        let path;
        if let Some(colon_index) = arg.find(':') {
            f_type = &arg[..colon_index];
            path = &arg[colon_index + 1..];
            if f_type!="ftp" && f_type != "zip" && f_type!="folder"
            {
                panic!("Ați introdus un tip de locație invalid");
            }
        } else {
            panic!("Formatul este: tip_fisier: path");
        }
        println!("Argumentul are indexul {index} Fisierul este de tipul {f_type} Calea este {}", path);
        locatii.push((f_type.to_string(),path.to_string()));
    }
    println!("Nr locatii= {}", locatii.len());
    initial_f(locatii);
}