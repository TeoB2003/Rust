use std::collections::HashMap;
use std::env;
use std::fs::{File, metadata};
//use std::env::current_dir;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
//use fs_extra::error::Error;
use zip::DateTime;
use zip::read::ZipArchive;
use zip::write::FileOptions;
use std::io::{self, prelude::*};
use std::path::Path;
use zip::{write::ZipWriter, CompressionMethod};
use chrono::{Utc, TimeZone, Datelike, Timelike};
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

fn sincronizare_f(locatii: Vec<(String,String)>)
{
    initial_f(locatii.clone());
    let mut vec_m:Vec<SystemTime>=Vec::new();
    //iau ultimii timpi
    for (_,b) in locatii.iter()
    {
        match fs::metadata(b) {
            Ok(metadata) => {
              let  modification_time=metadata.modified();
              vec_m.push(modification_time.unwrap());
              //println!("{:?}",modification_time.unwrap());
            }
            Err(e)=> println!("{}",e)
        }
    }
    let ok=1;
    while ok!=0 {    
        let mut  ind=0;
        for (_,b) in locatii.iter()
        {
            println!("{b}");
            match fs::metadata(b) {
                Ok(metadata) => {
                    if let Ok(modification_time) = metadata.modified() {
                        //println!("Ultima modificare a directorului {b}: {:?}", modification_time);
                        //println!("salve: {:?}",a_t);
                        let a_t=vec_m[ind];
                        match a_t.cmp(&modification_time) {
                            std::cmp::Ordering::Less => {
                                vec_m[ind]=modification_time;
                                println!("Ultima modificare a directorului {b}: {:?}", modification_time);
                               
                                let director_m=get_directory_contents(b).unwrap();
                                println!("E in directorul modificat {:?}",director_m);
                                for (_,o_folders) in locatii.iter()
                                    {
                                        if o_folders!=b
                                       {
                                        let curr_dir=get_directory_contents(&o_folders);
                                        println!("Directorul actual {:?}",curr_dir);
                                        for it in curr_dir.unwrap()
                                        {
                                            let  path=it.rsplit("\\").next().unwrap();
                                            let path1=(*b.clone()).to_string()+"\\"+path;
                                            println!("{path1}");
                                            if director_m.contains(&path1)==false
                                            {
                                                match fs::remove_file(it.clone()) {
                                                    Ok(_) => println!("Fișierul {it} a fost șters cu succes."),
                                                    Err(e) => eprintln!("Eroare la ștergerea fișierului: {}", e),
                                                }
                                            }
                                        } 
                                        }
                                   
                                }
                                println!("Apel la less!!!");
                                initial_f(locatii.clone());  //adauga fiserele noi
                                break;
                            }
                            std::cmp::Ordering::Equal => {
                            }
                            std::cmp::Ordering::Greater => {
                            }
                        }
                    } else {
                        println!("Nu s-a putut obține timpul ultimei modificări");
                    }
                    
                }
                Err(e) => {
                    println!("Eroare la obținerea metadatelor: {}", e);
                }
            }
            ind+=1;
        }
        initial_f(locatii.clone());
        std::thread::sleep(std::time::Duration::from_secs(3));     
    }
}


fn initial_f(loc: Vec<(String, String)>) {
    let mut index=0;
    let mut f_names: Vec<String>=Vec::new();
    let mut all_entries = HashMap::new();
    for (_, path) in loc.iter() {
        let last_component = path.rsplit('\\').next();
        match last_component {


            Some(component) => {f_names.push(component.to_string());},
            None => eprintln!("Path does not have a last component"), //nu se ajunge aici
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
                       // println!("locatia este {locv}");
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
                                continue;
                           //     println!("Next");
                            }
                            std::cmp::Ordering::Less => {
                             //   println!("Next");
                            }
                        }
                          
                    }
                    else{
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
    let mut index:usize=0;
    for (_,ceva) in loc.iter(){
       // println!("C {ceva}");
        let locatii=get_directory_contents(ceva).unwrap();
        for ( b,k) in all_entries.clone()
        {
            let found = locatii.iter().any(|s1| s1.contains(&b));
            if found && k== index{
          //      println!("Fisierul '{}' exista.", b);
            } else {
               let loc_s=loc[k].1.clone()+&b;
               let loc_d=ceva.clone()+&b; 
               //println!("Mut de la {loc_s} la {loc_d}");
               match fs::copy(loc_s, loc_d) {
                Ok(_) => {
                  //  println!("Fișierul a fost copiat cu succes.");
                }
                Err(e) => {
                    eprintln!("Eroare la copierea fișierului: {}", e);
                }
            }
            }
           
        }
         index+=1;
    }
 println!("All Entries: {:?}", all_entries);
 //return all_entries;
 //sincronizare_f(loc);
}

fn sincronizare_z(loc: Vec<(String, String)>)->Result<(), io::Error> {
    let mut ind=0;
    let a=loc.len();
    print!("{a}");
        while ind<a {
        if ind+1>=a{
            copy_files_between_archives(&loc[ind].1,&loc[0].1)?;
        }
        else
        {
            copy_files_between_archives(&loc[ind].1,&loc[ind+1].1)?;
        }
        ind+=1;
    }
    while ind<a-1
    {
        copy_files_between_archives(&loc[0].1,&loc[ind].1)?;
        ind+=1;
    }
    Ok(())
}

fn copy_files_between_archives(src_path: &str, dest_path: &str) -> Result<(), io::Error> {
    // Deschide arhiva sursă pentru citire
    let src_file = File::open(src_path)?;
    let mut src_archive = ZipArchive::new(src_file)?;

    // Deschide arhiva de destinație pentru scriere
    let dest_file = File::create(dest_path)?;
    let mut dest_archive = ZipWriter::new(dest_file);

    
    for i in 0..src_archive.len() {
        let mut src_file = src_archive.by_index(i)?;

        let file_name = src_file.name().to_string();
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Stored)
            .unix_permissions(src_file.unix_mode().unwrap_or(0o755));

        
        let dest_file_path = Path::new(dest_path).join(&file_name);

        //fisier exista il copii pe cel mai recent
        if dest_file_path.exists() {
            let src_time1: DateTime=src_file.last_modified();
            let dest_t1=metadata(&dest_file_path).unwrap().modified().unwrap();
            let dest_t=dest_t1.duration_since(UNIX_EPOCH).unwrap().as_micros();
            //unix=1.01.1970
            let dt=Utc.timestamp_micros(dest_t as i64).unwrap();
            let y2= src_time1.year(); //an sursa
            let y1=dt.year();
            let mut ok=1;
            if (y2 as i32)<y1
            {
                ok=0;
            }
            else if y1==(y2 as i32) && ((src_time1.month() as u32)<dt.month())
            {
                ok=0;
            }
            else if (src_time1.month() as u32)==dt.month() && (src_time1.day() as u32)<dt.day()
            {
                ok=0;
            }
            else if (src_time1.day() as u32)==dt.day() && (src_time1.hour() as u32)<dt.hour()
            {
                ok=0;
            } 
            else if (src_time1.hour() as u32)==dt.hour() && (src_time1.minute() as u32)<dt.minute(){ok=0;}
            else if (src_time1.minute() as u32)==dt.minute() && (src_time1.second() as u32)<dt.second()
            {
                ok=0;
            }
           //pp ca fisierul din sursa e mai nou
           if ok==1  
            {
                dest_archive.start_file(file_name.clone(), options)?;

            // Citește conținutul fișierului din arhiva sursă
            let mut buffer = Vec::new();
            src_file.read_to_end(&mut buffer)?;
            // Scrie conținutul în arhiva de destinație
            dest_archive.write_all(&buffer)?;

            println!("File {} copied to destination successfully.", file_name);
            }

            
        } else {
            dest_archive.start_file(file_name.clone(), options)?;

            // Citește conținutul fișierului din arhiva sursă
            let mut buffer = Vec::new();
            src_file.read_to_end(&mut buffer)?;

            // Scrie conținutul în arhiva de destinație
            dest_archive.write_all(&buffer)?;

            println!("File {} copied to destination successfully.", file_name);
        }
    }

    // Finalizează arhiva de destinație
    dest_archive.finish()?;
    Ok(())
}

fn main() {
    let fisiere: Vec<String> = env::args().collect();
    let mut locatii: Vec<(String,String)>=Vec::new();
    let mut loc_s="";
    let mut a=1;
    for (index, arg) in fisiere.iter().skip(1).enumerate() 
    {
        let f_type;
        let path;
        if let Some(colon_index) = arg.find(':') {
            f_type = &arg[..colon_index];
            if a==1{
                loc_s=f_type;
            }
            else if f_type!=loc_s {
                panic!("Se pot sincroniza doar locatii de acelasi tip!!!");
            }
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
        a+=1;
    }
    println!("Nr locatii= {}", locatii.len());
    if loc_s=="folder" {
          sincronizare_f(locatii.clone());
    }
    else if loc_s=="zip"
    {
        println!("Apelez zip!!");
        let ab=sincronizare_z(locatii.clone());
        match ab{
            Ok(())=> println!("Sincronizarea a avut succes!"),
            Err(e)=> println!("Eroarea este {}",e)
        }
    }
    else {
        println!("Nu s-a dezvoltat decat pentru foldere");
    }
  
}