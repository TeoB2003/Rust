//use core::time;
use std::collections::HashMap;
//use std::error::Error;
use std::fs::{File, metadata};
//use clap::error::Error;
//use clap::builder::Str;
use ftp::{FtpError, FtpStream};
use std::fs;
use std::time:: UNIX_EPOCH;
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
           //pp ca fisierul din sursa e mai nou
           if ok==1  
            {
                dest_archive.start_file(file_name.clone(), options)?;

            // Citește conținutul fișierului din arhiva sursă
            let mut buffer = Vec::new();
            src_file.read_to_end(&mut buffer)?;
            // Scrie conținutul în arhiva de destinație
            dest_archive.write_all(&buffer)?;

            //println!("File {} copied to destination successfully.", file_name);
            }

            
        } else {
            dest_archive.start_file(file_name.clone(), options)?;

            // Citește conținutul fișierului din arhiva sursă
            let mut buffer = Vec::new();
            src_file.read_to_end(&mut buffer)?;

            // Scrie conținutul în arhiva de destinație
            dest_archive.write_all(&buffer)?;

          //  println!("File {} copied to destination successfully.", file_name);
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
    //println!("Path download {}",b);
    let mut local_file = File::create(b.clone()).unwrap();
   // println!(" in download: {}",c);
    let  file_data1 = a.simple_retr(&c).unwrap();
    let file_data=file_data1.get_ref();
    let c_r=local_file.write_all(file_data);
    match c_r {
        Ok(())=>println!(" "),
        Err(e)=>println!("{e}"),
    }

}

fn f_z(path_f:String, path_z: String)->Result<(),io::Error> 
{

    println!("{}, {}",path_f,path_z);
    let src_file = File::create("C:\\Users\\bolot\\OneDrive\\Desktop\\b.zip")?;//se creaza arhiva noua!!!
    let mut src_archive = zip::ZipWriter::new(src_file);
    
    let dest_file = File::open(path_z.clone())?;
    let mut dest_archive = ZipArchive::new(dest_file)?;
    let name_f=path_f.rsplit('\\').next().unwrap();
    println!("Nume folder {}",name_f);
    // mut din path_z in src
    for i in 0..dest_archive.len() {
        let mut file = dest_archive.by_index(i)?;

        let options = FileOptions::default()
            .compression_method(CompressionMethod::Stored);
           
        let name_fz=file.name();
        if name_fz!=name_f
        {
        src_archive.start_file(file.name(), options)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        src_archive.write_all(&buffer)?;
        }
    }

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755); 

    let file_content = fs::read(path_f.clone()).unwrap();
    src_archive.start_file(name_f, options).unwrap();
    src_archive.write_all(&file_content).unwrap();
    src_archive.finish().unwrap();
    copy_files_between_archives("C:\\Users\\bolot\\OneDrive\\Desktop\\b.zip", &path_z.clone()).unwrap();
    Ok(())
}
fn z_f(path_f:String, path_z: String,name_f:String)->Result<(),io::Error>
{
    let zip_file = File::open(path_z.clone())?;
   // println!("Arhiva {}",path_z.clone());
   // println!("Fisier de mutat {name_f}");
    let mut zip_archive = ZipArchive::new(zip_file)?;
    let mut file_index: usize=0;
    for i in 0..zip_archive.len()
    {
        let file_entry1 = zip_archive.by_index(i)?;
     //   println!("Fisier {i} cu numele {}",file_entry1.name());
        if file_entry1.name()== name_f
        {
            file_index=i;
          //  println!("GASIT la {i}");
            break;
        }
    }
    let mut file_entry = zip_archive.by_index(file_index)?;
    //let fl=file_entry.name();
    //println!("Nume fisier in z_f {}",fl );
        let mut file_content = Vec::new();
        file_entry.read_to_end(&mut file_content)?;

        // Specify the destination path for the moved file
        let destination_path = path_f+&'\\'.to_string()+&name_f;
        // Write the file content to the destination folder
        let mut destination_file = File::create(destination_path)?;
        destination_file.write_all(&file_content)?;
    Ok(())
}
fn upload(a: &mut FtpStream,remote:String,remote_b:String,path1:String)
{
    println!("Upload cale: {} ",path1);
    let mut flo=File::open(path1).unwrap();
    let n_p=remote_b.to_string()+"/"+&remote;
    println!("Upload cale1: {} ",n_p.clone());
                //sc.transfer_type(FileType::Binary).unwrap();
    let err=a.put(&n_p, &mut flo);
    match err {
                    Ok(_)=>println!("YEEEI"),
                    Err(e)=>eprintln!("Eroare urcare fisier {e}"),
                }
}

fn ftp_z(a: &mut FtpStream, path_z:String,name_f:String)->Result<(),io::Error>
{
    let c=name_f.rsplit('/').next().unwrap();
    download(a, "C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string()+c, name_f.clone());
    f_z("C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string()+c, path_z)?;
    Ok(())

}

fn z_ftp(path_z:String,a: &mut FtpStream, f_n:String)->Result<(),io::Error>
{
    let name=f_n.rsplit('/').next().unwrap();
    println!("Am primit fisierul {name}");
    z_f("C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou".to_string(), path_z.clone(), name.to_string())?;
    let mut base=f_n.clone();
    base=base.replace(name, "");
    base=base[..base.len()-1].to_string();
    println!("base is {base}");
    upload(a, name.to_string(), base, "C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string()+name);
    Ok(())
}

fn connect(user:String,pass:String, url:String)-> FtpStream
{
    let mut sc=FtpStream::connect((url.clone(),21)).unwrap();
    let b=sc.login(&user, &pass); 
    match b{
        Ok(())=> println!("Yes"),
        Err(e)=>println!("Eroare la reconectare {e}"),
    };
     sc
}
fn sync(vec_l: Vec<(String,String)>){
    println!("{:?}",vec_l);
    let mut fisiere:HashMap<String,(usize,u64)>=HashMap::new();
    let mut ftp_r:HashMap<String,  (FtpStream, String)>=HashMap::new(); // cheie=fisier, val=(ftpS, path_ftp)
    for (ind,(type_l,loc)) in vec_l.clone().into_iter().enumerate()
    {
        if type_l=="ftp"
        {        
            let user = &loc[0..loc.find(':').unwrap()].to_string();
            let ind1 = &loc.find(':').unwrap();
            let password = &loc[*ind1 + 1..loc.find('@').unwrap()];
            let ii=loc.find('@').unwrap();
            let url = &loc[ii+1..loc.find('/').unwrap()].to_string();
            let path = &loc[loc.find('/').unwrap()..].to_string();
            let  sc=connect(user.to_string(), password.to_string(), url.to_string());
            ftp_r.insert(loc.clone(), (sc,path.clone()));
            let mut sc=connect(user.to_string(), password.to_string(), url.to_string());
            let p=Some(String::as_str(path));
            let fs= sc.nlst(p).unwrap();
            for nf in fs
            {
                let rfn=nf.rsplit('/').next().unwrap();
                println!("{}",rfn);
                if !fisiere.contains_key(rfn)
                {
                    let time_mn=sc.mdtm(&nf).unwrap().unwrap();
                    let nrsec=time_mn.num_seconds_from_unix_epoch();
                        fisiere.insert(rfn.to_string(), (ind, nrsec as u64));
                }
                else {
                    let time_mn=sc.mdtm(&nf).unwrap().unwrap();
                    let nrsec=time_mn.num_seconds_from_unix_epoch();
                    if nrsec as u64 > fisiere[rfn].1{
                        fisiere.insert(rfn.to_string(), (ind,nrsec as u64));
                    } 

                }
            }
        }

        else if type_l=="folder"
        {
            let list_ff=get_directory_contents(&loc).unwrap();
            for i in list_ff
            {
                println!("Continut f: {i}");
                let n_ff=i.rsplit('\\').next().unwrap();
                if !fisiere.contains_key(n_ff)
                {
                    let mt_f=fs::metadata(i.clone()).unwrap();
                    let md=mt_f.modified().unwrap();
                    let duration_since_epoch = md.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
                    let tt=duration_since_epoch.as_secs();
                    fisiere.insert(n_ff.to_string(), (ind,tt));
                }
                else{
                    let ind_g= fisiere[n_ff].0;
                    let type_l=vec_l.get(ind_g).unwrap().0.clone();
                    println!("Gasit in {}",type_l);
                    let mt_f=fs::metadata(i.clone()).unwrap();
                    let md=mt_f.modified().unwrap();
                    let duration_since_epoch = md.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
                    let tt=duration_since_epoch.as_secs();
                    if tt> fisiere[n_ff].1{
                        fisiere.insert(n_ff.to_string(), (ind,tt));
                    }  
                }
            }

        }
        else if type_l=="zip"
        {
            let src_file = File::open(loc);
            let mut src_archive = ZipArchive::new(src_file.unwrap()).unwrap();
            for xy in 0..src_archive.len()
            {
               
                let  zip_file = src_archive.by_index(xy).unwrap();
                let n=zip_file.name().to_string();
                 let zip_time: zip::DateTime=zip_file.last_modified();
                 let t=Utc.with_ymd_and_hms(zip_time.year() as i32, zip_time.month() as u32, zip_time.day() as u32, zip_time.hour() as u32, zip_time.minute() as u32, zip_time.second() as u32);
                let aux=t.unwrap().timestamp();
                if !fisiere.contains_key(&n)
                {
                    fisiere.insert(n, (ind,aux as u64));
                }
                else {
                    let val=fisiere[&n];
                    if aux as u64> val.1
                    {
                        fisiere.insert(n, (ind,aux as u64)); 
                    }
                }
            }
        }
    } 
    println!("{:?}",fisiere);
    //initializare
    for j in &fisiere
    {
        for (ind,a) in vec_l.iter().enumerate()
        {
            if ind!=j.1.0   
            {    
                if a.0=="ftp"{  //fac upload in ftp  
                    if vec_l[j.1.0].0=="folder"{
                        let  ftp_d= ftp_r.get_mut(&a.1.clone()).unwrap();
                        let  aux=  &mut ftp_d.0;
                        let path=ftp_d.1.clone();
                        let f_n=j.0.clone();
                        let path1=vec_l[j.1.0].1.clone()+&'\\'.to_string()+&f_n;
                        //println!("Path= {path1}");
                        upload(aux, j.0.clone(), path.to_string(), path1);
                    }
                    else if vec_l[j.1.0].0=="zip"{
                        println!("Upload din zip in ftp");
                        let path_z1=&vec_l[j.1.0].1;
                        println!("Zip= {}",path_z1);
                        println!("a1= {}",a.1.clone());
                        let  ftp_d= ftp_r.get_mut(&a.1.clone()).unwrap();
                        let  aux=  &mut ftp_d.0;
                        let ftp_p=ftp_d.1.clone()+&'/'.to_string()+j.0;
                        println!("Nume file de mutat in ftp {}",ftp_p);
                        z_ftp(path_z1.to_string(), aux, ftp_p).unwrap();
                    }
                    else if vec_l[j.1.0].0=="ftp"
                    {

                    }
                }  
                else if a.0=="zip"{
                    println!("Fisierul {}",j.0);
                    
                    if vec_l[j.1.0].0=="folder"
                    {
                        let path_f=vec_l[j.1.0].1.clone()+&'\\'.to_string()+&j.0;
                        let path_z=a.1.clone();
                        let er=f_z(path_f.clone(), path_z);
                        match er{
                            Ok(())=>println!("Succes fisier la zip!! {}",path_f.clone()),
                            Err(e)=>println!("Eroare la sin: {e}"),
                        }
                    }
                    else if vec_l[j.1.0].0=="ftp"
                    {
                        let  ftp_d= ftp_r.get_mut(&vec_l[j.1.0].1.clone()).unwrap();
                        let  aux=  &mut ftp_d.0;
                        let path_ftp=ftp_d.1.clone()+&'/'.to_string()+j.0;
                        ftp_z(aux, a.1.clone(), path_ftp).unwrap();
                    }
                    else if vec_l[j.1.0].0=="zip"
                    {

                    }
                } 
                else if a.0 =="folder"{
                    println!("Fisierul {}",j.0);
                    if vec_l[j.1.0].0=="ftp"
                    {
                        let ftp_d=ftp_r.get_mut(&vec_l[j.1.0].1.clone()).unwrap();
                        let aftp=&mut ftp_d.0;
                        let b= a.1.clone()+&'\\'.to_string()+&j.0.clone();
                        let ax=ftp_d.1.clone();
                        let c=ax+&'/'.to_string()+&j.0.clone();
                        download(aftp, b, c);
                    }
                    else if vec_l[j.1.0].0=="zip"
                    {
                        let path_f=a.1.clone();
                        let path_z=vec_l[j.1.0].1.clone();
                        let er=z_f(path_f, path_z, j.0.clone());
                        match er{
                            Ok(())=>println!("Succes cu {}",j.0),
                            Err(e)=>println!("Eroare la sincronizare initiala: {e}"),
                        }
                    }
                    else if vec_l[j.1.0].0=="folder"
                    {
                        let to=a.1.clone()+&'\\'.to_string()+j.0;
                        let from=vec_l[j.1.0].1.clone()+&'\\'.to_string()+j.0;
                        fs::copy(from, to).unwrap();
                    }
                }
            }
        }
    }

     loop {
        for (i1,i) in vec_l.iter().enumerate()
        {
            let mut fis_add:String=String::new();
            let mut index_add=0;
            let mut fis_del:String=String::new();
            if i.0=="folder"
            {
                let fs_f=get_directory_contents(&i.1).unwrap();
                for file_f in &fs_f //s-a modificat un file sau s-a adaugat in !folder!
                {
                    let file_name=file_f.rsplit('\\').next().unwrap();
                    if !fisiere.contains_key(file_name)
                    {
                        fis_add=file_f.clone();
                        let mt_f=fs::metadata(file_f.clone()).unwrap();
                        let md=mt_f.modified().unwrap();
                        let duration_since_epoch = md.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
                        let tt=duration_since_epoch.as_secs();
                        fisiere.insert(file_name.to_string(), (i1,tt));
                        break;
                    }
                    else {
                       // println!("Fisier= {}",file_f);
                        let mt_f=fs::metadata(file_f.clone()).unwrap();
                        let md=mt_f.modified().unwrap();
                        let duration_since_epoch = md.duration_since(UNIX_EPOCH).expect("SystemTime before UNIX EPOCH!");
                        let tt=duration_since_epoch.as_secs();
                        let ot=fisiere[file_name].1;
                        if tt>ot
                        {
                            fis_add=file_f.clone();
                            fisiere.insert(file_name.to_string(), (i1,tt));
                            break;
                        }
                    }
                }

                for in_f in &fisiere
                {
                    let mut ok=1;
                   for ad in &fs_f  //caut sa vad daca nu s-a sters un fisier
                   {
                     let nn=ad.rsplit('\\').next().unwrap();
                     if in_f.0==nn
                     {
                        ok=0;
                     }
                   }
                   if ok==1
                   {
                    fis_del=in_f.0.to_string();
                    fisiere.remove(&in_f.0.clone());
                    break;
                   }
                }
                if !fis_add.is_empty()
                {
                    for ind in vec_l.iter()
                    {
                        let name_f=fis_add.rsplit('\\').next().unwrap();
                        let cmp_n=ind.1.clone()+&'\\'.to_string()+name_f;
                        if ind.0=="folder" && fis_add!=cmp_n //sa nu adaug in acelasi fisier
                        {
                            println!("S-a adaugat {}" ,fis_add);
                            println!("{}",ind.1);
                            fs::copy(fis_add.clone(), cmp_n.clone()).unwrap();
                        }
                        else if ind.0=="ftp"
                        {
                            let a1=ftp_r.get_mut(&ind.1).unwrap();
                            let a=&mut a1.0;
                            let remote_b=a1.1.clone();
                            let remote=fis_add.rsplit('\\').next().unwrap();
                            upload(a, remote.to_string(), remote_b, fis_add.clone());
                        }
                        else if ind.0=="zip"
                        {
                            f_z(fis_add.clone(), ind.1.clone()).unwrap();
                        }
                    }
                } 
                else if !fis_del.is_empty()
                {
                    for ind in &vec_l
                    {
                        if ind.0=="folder"
                        {
                            let path_r=ind.1.clone()+&'\\'.to_string()+&fis_del;
                            println!("Path stergere ftp {}",path_r);
                            if ind.1!=i.1
                            {fs::remove_file(path_r).unwrap();}
                        }
                        else if ind.0=="ftp"
                        {
                            let ftp_a=ftp_r.get_mut(&ind.1).unwrap();
                            let ftp_d=&mut ftp_a.0;
                            let path_del=ftp_a.1.clone()+&'/'.to_string()+&fis_del;
                            ftp_d.rm(&path_del).unwrap();
                        }
                        else if ind.0=="zip"  //stergere in zip
                        {
                            let src_path=ind.1.clone();
                            let e_file = File::open(src_path).unwrap();
                            let mut e_archive = ZipArchive::new(e_file).unwrap();

                            let new_f=File::create("C:\\Users\\bolot\\OneDrive\\Desktop\\c.zip").unwrap();
                            let mut n_arch=zip::ZipWriter::new(new_f);

                            for i23 in 0..e_archive.len() {
                                let mut file = e_archive.by_index(i23).unwrap();
                        
                                // Skip the file to delete
                                if file.name() == fis_del {
                                    continue;
                                }
                        
                                let options = FileOptions::default()
                                    .compression_method(CompressionMethod::Stored);
                                   
                        
                                n_arch.start_file(file.name(), options).unwrap();
                        
                                let mut buffer = Vec::new();
                                file.read_to_end(&mut buffer).unwrap();
                        
                                n_arch.write_all(&buffer).unwrap();
                            }
                            n_arch.finish().unwrap();
                            copy_files_between_archives("C:\\Users\\bolot\\OneDrive\\Desktop\\c.zip", &ind.1.clone()).unwrap();
                        }
                    }
                }
            }
            else if i.0=="ftp"
            {
                let ftp_a=ftp_r.get_mut(&i.1).unwrap();
                let ftp_d=&mut ftp_a.0;
                let pathname=Some(String::as_str(&ftp_a.1));
                let lista_file=ftp_d.nlst(pathname).unwrap();
                for i in &lista_file
                {
                    let name_i=i.rsplit('/').next().unwrap();
                    if !&fisiere.contains_key(name_i)
                    {
                        index_add=i1;
                        fis_add=name_i.to_string();
                        let ptt=i;
                        println!("Path ptt= {ptt}");
                        let time_ft=ftp_d.mdtm(ptt).unwrap().unwrap();
                        let time_ftp=time_ft.num_seconds_from_unix_epoch() as u64;
                        fisiere.insert(name_i.to_string(), (i1,time_ftp));
                        break;
                    }
                    else {
                        let ptt=i;
                        let time_ft=ftp_d.mdtm(ptt).unwrap().unwrap();
                        let time_ftp=time_ft.num_seconds_from_unix_epoch() as u64;
                        let o_t=fisiere[name_i].1;
                        //println!("{o_t}{time_ftp}");
                        if o_t<time_ftp
                        {
                            println!("Modificare in ");
                            fis_add=name_i.to_string();
                            fisiere.insert(name_i.to_string(), (i1,o_t));
                            break;
                        }
                    }
                }
                for ij in &fisiere
                {
                    let mut ok_g=1;
                    for tt in &lista_file
                    {
                        let n_tt=tt.rsplit('/').next().unwrap();
                        if n_tt.to_string()==*ij.0
                        {
                            ok_g=0;
                            break;
                        }
                    }
                    if ok_g==1
                    {
                        fis_del=ij.0.to_string();
                        fisiere.remove(&fis_del);
                        break;
                    }
                } 
                if !fis_add.is_empty()
                {
                    for (in1,nsh) in vec_l.iter().enumerate()
                    {
                        if in1!=i1
                        {
                            if nsh.0=="folder"
                            {
                                let path_f=vec_l[index_add].1.clone();
                                let ftp_dm=ftp_r.get_mut(&path_f).unwrap();
                                let p_ftp=ftp_dm.1.clone()+&'/'.to_string()+&fis_add;
                                let a= &mut ftp_dm.0;
                                let b=nsh.1.clone()+&'\\'.to_string()+&fis_add;
                                println!("b={b}");
                                download(a, b, p_ftp);
                            }
                            else if nsh.1=="zip"
                            {

                            }
                        }
                    }
                }
                else if !fis_del.is_empty() {
                    for (in1,nsh) in vec_l.iter().enumerate()
                    {
                        if in1!=i1
                        {
                            if nsh.0=="folder"
                            {
                                let b=nsh.1.clone()+&'\\'.to_string()+&fis_del;
                                fs::remove_file(b).unwrap();
                            }
                            else if nsh.1=="zip"
                            {

                            }
                        }
                    }
                    
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));  //nu trebuie mers in zip, e read only!!(adaug la siguranta la final)
        }
        std::thread::sleep(std::time::Duration::from_secs(1)); 
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
    let mut ok=0;
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
                    ok=1;
            }
            path = &arg[colon_index + 1..];
            if f_type!="ftp" && f_type != "zip" && f_type!="folder"
            {
                panic!("Se pot da doar locatii ftp, folder sau zip");
            }
        } else {
            panic!("Formatul este: tip_fisier: path");
        }
        println!("Argumentul are indexul {index} Fisierul este de tipul {f_type} Calea este {}", path);
        locatii.push((f_type.to_string(),path.to_string()));
        a+=1;
    }
    println!("Nr locatii= {}", locatii.len());
    if ok==1 || loc_s=="folder"
    {
        println!("Apel folder");
        sync(locatii.clone());
    }
    else{
     if loc_s=="zip"
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
}
  