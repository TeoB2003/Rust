use std::collections::HashMap;
//use std::error::Error;
use std::fs::{File, metadata};
//use clap::builder::Str;
use ftp::{FtpError, FtpStream};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use zip::DateTime;
use zip::read::ZipArchive;
use zip::write::FileOptions;
use std::io::{self, prelude::*};
//use ftp::types::FileType;
use std::path::Path;
use zip::{write::ZipWriter, CompressionMethod};
use chrono::{ TimeZone, Datelike, Timelike,Utc};
fn get_directory_contents(directory_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut entries = Vec::new();

    if let Ok(entries_iter) = fs::read_dir(directory_path) {
        for entry in entries_iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                if !path.is_dir() {
                    entries.push(path.display().to_string());
                } 
            }
            else { return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error reading directory: {}", directory_path),));}
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
    loop {    
        for (ind,(_,b)) in locatii.iter().enumerate()
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
                                        let curr_dir=get_directory_contents(o_folders);
                                        println!("Directorul actual {:?}",curr_dir);
                                        for it in curr_dir.unwrap()
                                        {
                                            let  path=it.rsplit('\\').next().unwrap();
                                            let path1=(*b.clone()).to_string()+"\\"+path;
                                            println!("{path1}");
                                            if !director_m.contains(&path1)
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
    for (index, (_,ceva)) in loc.iter().enumerate(){
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
            let dt=Utc.timestamp_micros(dest_t as i64).unwrap();
            let y2= src_time1.year(); //an sursa
            let y1=dt.year();
            let mut ok=1;
            if (y2 as i32)<y1 || (y1==(y2 as i32) && ((src_time1.month() as u32)<dt.month())) || ((src_time1.month() as u32)==dt.month() && (src_time1.day() as u32)<dt.day())
            ||((src_time1.day() as u32)==dt.day() && (src_time1.hour() as u32)<dt.hour()) ||((src_time1.hour() as u32)==dt.hour() && (src_time1.minute() as u32)<dt.minute())
            ||((src_time1.minute() as u32)==dt.minute() && (src_time1.second() as u32)<dt.second())
            {
                ok=0;
            }
            /*else if y1==(y2 as i32) && ((src_time1.month() as u32)<dt.month())
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
            }*/
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

fn sincronizare_ftp(loc: Vec<(String, String)>)-> Result<(),FtpError>
{
    let mut fisiere:HashMap<String,usize>=HashMap::new();
    let mut user_v:Vec<String>=Vec::new();
    let mut  parole_v:Vec<String>=Vec::new();
    let mut url_v:Vec<String>=Vec::new();
    let mut path_v:Vec<String>=Vec::new();
    let mut  index=0;
    //prelucrez input
    while index<loc.len() {
        println!("{}",loc[index].1);
        let ind=&loc[index].1.find(':').unwrap();
        user_v.push((loc[index].1[0..*ind]).to_string());
        let ind1=&loc[index].1[*ind+1..].find('@').unwrap();
        
        parole_v.push(loc[index].1[*ind+1..*ind+*ind1+1].to_string());
        let ind2=&loc[index].1[*ind1+*ind+2..].find('/').unwrap();
        url_v.push(loc[index].1[*ind1+*ind+2..*ind1+*ind+2+ind2].to_string());
        path_v.push(loc[index].1[*ind1+*ind+2+*ind2..].to_string());
        index+=1;
    }
    let mut ftp_v:Vec<FtpStream>=Vec::new();
    index=0;
    while index< loc.len() //conectare la servere
    {
    match FtpStream::connect((url_v[index].clone(),21)) {
        Ok( mut ftp_stream) => {
            // Autentificare cu nume de utilizator și parolă
            if let Err(err) = ftp_stream.login(&user_v[index], &parole_v[index]) {
                eprintln!("Failed to log in to {}: {:?}",url_v[0].clone(), err);
            }
             else {
                println!("Connection successful and logged in to {}!", url_v[0]);
                let p=Some(String::as_str(&path_v[index]));
            let source_files=ftp_stream.list(p);
                ftp_v.push(ftp_stream);
                for it in source_files.unwrap()
                {
                    println!("It ={:?}",it);
                    let path_i=path_v[index].to_string()+"/"+it.rsplit(' ').next().unwrap();
                    if !it.starts_with('d')
                    {
                    let a=ftp_v[index].mdtm(&path_i).unwrap();
                    if fisiere.contains_key(it.rsplit(' ').next().unwrap())
                    {
                        let index_n=fisiere[it.rsplit(' ').next().unwrap()];
                        let c_n=path_v[index_n].clone()+"/"+it.rsplit(' ').next().unwrap();
                        let b=ftp_v[index_n].mdtm(&c_n).unwrap();
                        println!("A= {:?} si B={:?}",a,b);
                        if a>b
                        {
                            fisiere.insert(it.rsplit(' ').next().unwrap().to_string(), index);
                        }
                    }
                    else{
                    fisiere.insert(it.rsplit(' ').next().unwrap().to_string(), index);
                    }
                    }

                }
                //println!("{:?}",source_files.unwrap());
            }
        }
        Err(err) => {
            eprintln!("Failed to connect to {}: {:?}", url_v[0].clone(), err);
        }
    }
    index+=1;
    //let source_files=ftp_stream1.list(None);
}
    //index=0;
    println!("Fisierele toate {:?}",fisiere);

    let mut fisiere_l: Vec<String>=Vec::new();
    for il in fisiere.iter()
    {
        let path_l="C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string();
        
        let server_s: &mut FtpStream=&mut ftp_v[*il.1];
        let local_path=path_l.clone()+il.0;
        let mut local_file = File::create(local_path.clone()).unwrap();
        let sp=path_v[*il.1].clone()+"/"+il.0;
        println!("{}",sp);
        let  file_data1 = server_s.simple_retr(&sp).unwrap();
        let file_data=file_data1.get_ref();
        let c_r=local_file.write_all(file_data);
        match c_r {
            Ok(())=>println!(" "),
            Err(e)=>println!("{e}"),
        }
        fisiere_l.push(local_path.clone());

    }
//initial sincronizare locatii
   let mut inbex=0;
   let lg=ftp_v.len();
    while inbex<lg
    {
        for i in fisiere_l.clone()
        {
            println!("Sunt in locatia {inbex}");
            println!("{i}");
            let mut flo=File::open(i.clone()).unwrap();
              let rf=i.rsplit('\\').next().unwrap();
              if inbex!=fisiere[rf]
              {
                println!("{rf}");
                let mut sc=FtpStream::connect((url_v[inbex].clone(),21)).unwrap();
                let b=sc.login(&user_v[inbex], &parole_v[inbex]); 
                match b{
                    Ok(())=> println!("Yes"),
                    Err(e)=>println!("Eroare la reconectare {e}"),
                };
                println!("Locatia folder nou {}",i);
                let n_p=path_v[inbex].to_string()+"/"+rf;
                //sc.transfer_type(FileType::Binary).unwrap();
                let err=sc.put(&n_p, &mut flo);
                match err {
                    Ok(_)=>println!("YEEEI"),
                    Err(e)=>eprintln!("Eroare urcare fisier {e}"),
                }
              }
        }
        inbex+=1;
    }


    let mut v_n_f:Vec<String>=Vec::new();
            for i in fisiere.keys()
            {
                let c=i.rsplit('\\').next().unwrap().to_string();
                println!("Acesta este {c}");
                v_n_f.push(c);
            }
    loop{
        let mut i=0;
        let lg=ftp_v.len();
        while i<lg
        {
            println!("{},{}",user_v[i],parole_v[i]);
            let mut sc=FtpStream::connect((url_v[i].clone(),21)).unwrap();
            let b=sc.login(&user_v[i], &parole_v[i]); 
            match b{
                Ok(())=> println!("Yes"),
                Err(e)=>println!("Eroare la reconectare {e}"),
            };
            let as1=Some(String::as_str(&path_v[i]));
            let s_files=sc.nlst(as1).unwrap();
            println!("Serverul {i} are {:?}",s_files);

            let mut  dif_f_s:Vec<String>=Vec::new();
            for j1 in v_n_f.iter()
            {
                let mut ok=0;
                for b1 in s_files.iter()
                {
                    if j1== b1.rsplit('/').next().unwrap()
                    {
                        ok=1;
                    }
                }
                if ok==0
                {
                    dif_f_s.push(j1.to_string());
                }
            }
            let mut dif_s_f:Vec<String>=Vec::new();
            for b1 in s_files.iter()
            {
                let mut ok=0;
                for j1 in v_n_f.iter()
                {
                    if j1== b1.rsplit('/').next().unwrap()
                    {
                        ok=1;
                    }
                }
                if ok==0
                {
                    dif_s_f.push(b1.to_string());
                }
            }
            println!("Diferenta1  este {:?} si dif 2 e {:?}",dif_f_s,dif_s_f);
            if !dif_f_s.is_empty() //s-a sters un fisier
            {
                println!("S-a sters un fisier: {}",dif_f_s[0]);
                let mut pf_s:String=String::from("");
                for is in fisiere_l.clone()
                {
                    if is.contains(&dif_f_s[0].clone())
                    {
                        pf_s=is;
                        break;
                    }
                }
                println!("Path stergere: {}",pf_s);
                match fs::remove_file(pf_s)
                {
                    Ok(_)=> println!(),
                    Err(e)=>println!("Eroare stergere {}",e),
                }
                v_n_f.retain(|elem| elem != &dif_f_s[0]);
                let mut j=0;
                while j<ftp_v.len()
                {
                    if j!=i{
                        let mut ftp_stream = FtpStream::connect((url_v[j].clone(), 21))?;
                        ftp_stream.login(&user_v[j], &parole_v[j])?;
                    
                        let p_s=path_v[j].clone()+"/"+&dif_f_s[0];
                        println!("Vreau sa sterg de pe server {}",p_s);
                        ftp_stream.rm(&p_s)?;
                    }
                    j+=1;
                }
                println!("Fisiere dupa stergere {:?}",v_n_f);
                fisiere.remove(&dif_f_s[0]);
            }
            else if !dif_s_f.is_empty()
            { //s-a adugat
                v_n_f.push(dif_s_f[0].rsplit('/').next().unwrap().to_string());
                println!("{:?}",v_n_f);
                let path_l="C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string();
                let server_s: &mut FtpStream=&mut ftp_v[i];
                let local_path=path_l.clone()+dif_s_f[0].rsplit('/').next().unwrap();
                //println!("{:?}",local_path);
                download(server_s,local_path.clone(),dif_s_f[0].clone());
                for (ind,it_s) in ftp_v.iter_mut().enumerate() 
                {
                        if ind!=i{
                        println!("Fac upload in {ind}");
                        upload(it_s,dif_s_f[0].rsplit('/').next().unwrap().to_string(),path_v[ind].clone(),local_path.clone());
                        }
                }
                fisiere.insert(dif_s_f[0].rsplit('/').next().unwrap().to_string(), i);
            }
            else {
                println!("Egal");
                for (i1,v) in fisiere.clone()
                {
                    let path_a=path_v[i].clone()+"/"+&i1;
                    println!("Primul {}",path_a);
                    let t1=ftp_v[i].mdtm(&path_a).unwrap();
                    let path_b=path_v[v].clone()+"/"+&i1;
                    println!( "Doi {}",path_b);
                    let t2=ftp_v[v].mdtm(&path_b).unwrap();
                    if t1>=t2 {
                        println!("E mai nou {i}");
                        std::thread::sleep(std::time::Duration::from_secs(1)); 
                        let l_p23="C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string()+&i1;
                        println!("Download ");
                        download(&mut ftp_v[i], l_p23.clone(), path_a);
                        fisiere.insert(i1.to_string().clone(),i);
                        for (iss,zz) in ftp_v.iter_mut().enumerate()
                        {
                            upload(zz, i1.clone(), path_v[iss].clone(), l_p23.clone());
                        }
                    }
                }
            }
            i+=1;

        }
        std::thread::sleep(std::time::Duration::from_secs(3)); 
    }
    //Ok(())
}
fn download(a: &mut FtpStream,b:String ,c: String  )
{
    let mut local_file = File::create(b.clone()).unwrap();
    let  file_data1 = a.simple_retr(&c).unwrap();
    let file_data=file_data1.get_ref();
    let c_r=local_file.write_all(file_data);
    match c_r {
        Ok(())=>println!(" "),
        Err(e)=>println!("{e}"),
    }

}


fn upload(a: &mut FtpStream,remote:String,remote_b:String,path1:String)
{
    let mut flo=File::open(path1).unwrap();
    let n_p=remote_b.to_string()+"/"+&remote;
                //sc.transfer_type(FileType::Binary).unwrap();
    let err=a.put(&n_p, &mut flo);
    match err {
                    Ok(_)=>println!("YEEEI"),
                    Err(e)=>eprintln!("Eroare urcare fisier {e}"),
                }
}
use clap::Parser;
#[derive(Parser)]
#[derive(Debug)]
#[clap(author="Teofil Bolotă", version, about="Se vor da argumente de tipul: ftp:user:password@URL/a.b.c sau  zip:C:\\abc\\d.zip sau folder:C:\\aaa")]
struct Argument{
    files:Vec<String>,
}
fn main() {
    let arg=Argument::parse();
    println!("{:?}",arg);
    let fisiere: Vec<String> = arg.files;
    let mut locatii: Vec<(String,String)>=Vec::new();
    let mut loc_s="";
    let mut a=1;
    for (index, arg) in fisiere.iter().enumerate() 
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
        println!("Am ftp-uri");
        let ab=sincronizare_ftp(locatii.clone());
        match ab{
            Err(e)=>println!("Eroare {}",e),
            Ok(())=> print!("s")
        }   
    }
}
  