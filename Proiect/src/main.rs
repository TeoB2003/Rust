use std::collections::HashMap;
use std::fs::{File, metadata};
use ftp:: FtpStream;
use std::fs;
use std::time:: UNIX_EPOCH;
use zip::DateTime;
use zip::read::ZipArchive;
use zip::write::FileOptions;
use std::io::{self, prelude::*};
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
                    else if vec_l[j.1.0].0=="ftp"  //din j in a
                    {
                        let path_l="C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string();
                        let b= path_l+&'\\'.to_string()+j.0;
                                let pathf=vec_l[j.1.0].1.clone();
                                let server_s1=ftp_r.get_mut(&pathf).unwrap();
                                let server_s=&mut server_s1.0;
                                let c=server_s1.1.clone()+&'/'.to_string()+j.0;
                                //println!("{:?}",local_path);
                                download(server_s,b.clone(),c);
                                let server_d1= ftp_r.get_mut(&a.1).unwrap();
                                let server_d=&mut server_d1.0;
                                let bb=server_d1.1.clone();
                                upload(server_d, j.0.clone(),bb, b);
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
                        //din j in a
                        let src_file = File::create("C:\\Users\\bolot\\OneDrive\\Desktop\\c.zip").unwrap();//se creaza arhiva noua!!!
                        let mut src_archive = zip::ZipWriter::new(src_file);
                        
                        //deschid a
                        let dest_file = File::open(a.1.clone()).unwrap();
                        println!("Arhiva a {}",a.1);
                        let mut dest_archive = ZipArchive::new(dest_file).unwrap();

                        let name_f=j.0.clone();
                        println!("Nume folder {}",name_f);

                        for i in 0..dest_archive.len() {
                            let mut file = dest_archive.by_index(i).unwrap();
                    
                            let options = FileOptions::default()
                                .compression_method(CompressionMethod::Stored);
                               
                            let name_fz=file.name();
                            if name_fz!=name_f
                            {
                            src_archive.start_file(file.name(), options).unwrap();
                    
                            let mut buffer = Vec::new();
                            file.read_to_end(&mut buffer).unwrap();
                    
                            src_archive.write_all(&buffer).unwrap();
                            }
                        }
                        //a e mutat in c

                         let dest_file_j = File::open(vec_l[j.1.0].1.clone()).unwrap();
                        let mut dest_archive_j = ZipArchive::new(dest_file_j).unwrap();

                        for i in 0..dest_archive_j.len() {
                            let mut file = dest_archive_j.by_index(i).unwrap();
                    
                            let options = FileOptions::default()
                                .compression_method(CompressionMethod::Stored);
                               
                            let name_fz=file.name();
                            if name_fz==name_f
                            {
                            src_archive.start_file(file.name(), options).unwrap();
                    
                            let mut buffer = Vec::new();
                            file.read_to_end(&mut buffer).unwrap();
                    
                            src_archive.write_all(&buffer).unwrap();
                            break;
                            }
                        }
                        src_archive.finish().unwrap();
                        println!("zip {}",a.1);
                        copy_files_between_archives("C:\\Users\\bolot\\OneDrive\\Desktop\\c.zip", &a.1.clone()).unwrap();
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
            println!("Sunt in {}",i.1);
            println!("{:?}",fisiere);
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
                            println!("S-a adaugat in folder la folder {}" ,fis_add);
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
                            println!("UPDATE");
                            f_z(fis_add.clone(), ind.1.clone()).unwrap();
                        }
                    }
                } 
                if !fis_del.is_empty()
                {
                    for ind in &vec_l
                    {
                        if ind.0=="folder"
                        {
                            let path_r=ind.1.clone()+&'\\'.to_string()+&fis_del;
                            println!("Path stergere folder {}",path_r);
                            if ind.1!=i.1
                            {fs::remove_file(path_r).unwrap();}
                        }
                        else if ind.0=="ftp"
                        {
                            let ftp_a=ftp_r.get_mut(&ind.1).unwrap();
                            let ftp_d=&mut ftp_a.0;
                            let path_del=ftp_a.1.clone()+&'/'.to_string()+&fis_del;
                            println!("Vreau sa sterg: {path_del}");
                            ftp_d.rm(&path_del).unwrap();
                        }
                        else if ind.0=="zip"  //stergere in zip
                        {
                            println!("Sterg {}",fis_del);
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
                        println!("Timpii {o_t} si {time_ftp}");
                        if o_t+1<time_ftp
                        {
                            println!("Modificare in {ptt}");
                            fis_add=name_i.to_string();
                            fisiere.insert(name_i.to_string(), (i1,time_ftp));
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
                        if n_tt==ij.0
                        {
                            ok_g=0;
                            break;
                        }
                    }
                    if ok_g==1
                    {
                        println!("S-a sters fisier {}",fis_del);
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
                            else if nsh.0=="zip"
                            {
                                let path_z=vec_l[i1].1.clone();
                                println!("path_f {path_z}");
                                let ftp_dm=ftp_r.get_mut(&path_z).unwrap();
                                let p_ftp=ftp_dm.1.clone()+&'/'.to_string()+&fis_add;
                                let a= &mut ftp_dm.0;
                                ftp_z(a, nsh.1.clone(), p_ftp.clone()).unwrap();
                            }
                            else if nsh.0=="ftp"
                            {
                                let path_l="C:\\Users\\bolot\\OneDrive\\Desktop\\Folder nou\\".to_string();
                                let b= path_l+&'\\'.to_string()+&fis_add;
                                let pathf=vec_l[i1].1.clone();
                                let server_s1=ftp_r.get_mut(&pathf).unwrap();
                                let server_s=&mut server_s1.0;
                                //am deschis server cu fisier recent
                                let c=server_s1.1.clone()+&'/'.to_string()+&fis_add;
                                //println!("{:?}",local_path);
                                download(server_s,b.clone(),c);
                                let server_d1= ftp_r.get_mut(&nsh.1).unwrap();
                                let server_d=&mut server_d1.0;
                                let bb=server_d1.1.clone();
                                upload(server_d, fis_add.clone(),bb, b);
                            }
                        }
                    }
                }
          if !fis_del.is_empty() {
                
                    for (in1,nsh) in vec_l.iter().enumerate()
                    {
                        if in1!=i1
                        {
                            if nsh.0=="folder"
                            {
                                let b=nsh.1.clone()+&'\\'.to_string()+&fis_del;
                                fs::remove_file(b).unwrap();
                            }
                            else if nsh.0=="zip" //stergere in zip
                            {
                                let src_path=nsh.1.clone();
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
                                copy_files_between_archives("C:\\Users\\bolot\\OneDrive\\Desktop\\c.zip", &nsh.1.clone()).unwrap();

                            }
                            else if nsh.0=="ftp" {
                                let server_d1= ftp_r.get_mut(&nsh.1).unwrap();
                                let server_d=&mut server_d1.0;
                                let bas=server_d1.1.clone();
                                let path_sters=bas.clone()+&'/'.to_string()+&fis_del;
                                println!("Path sters= {path_sters}");
                                server_d.rm(&path_sters).unwrap();
                            }
                        }
                    }
                    
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));  
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
    if ok==1 || loc_s=="folder" || loc_s=="ftp" || loc_s=="zip"
    {
        println!("Apel folder");
        sync(locatii.clone());
    }
}
  