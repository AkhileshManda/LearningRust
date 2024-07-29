use std::fs;
use std::io;

fn main() {
    //This is a common pattern used for many examples
    //Exit code 0 is normal else there is an error and the program exited
    std::process::exit(extract_zip());
}

fn extract_zip() -> i32 {
    //collect the list (vector) of arguments from the commmand line
    // '_' is used to infer type which in this case will be a string
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage : {} <filename>", args[0]);
        return 1;
    }
    
    //the name of the zip file
    let fname = std::path::Path::new(&*args[1]);
    //the file
    let file = fs::File::open(&fname).unwrap();
    //using the crate to iterate through the zip file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // going through each file in the zip file
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        
        // getting the path of the file in the zip
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment(); //commment of the file ie filename
            if !comment.is_empty() {
                println!("File {} comment:{}", i, comment);
            }
        }
        // if we encounter a folder then we have to create a folder in our output
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to {}", i, outpath.display());
            // use the fs crate to create folder at the req path
            // unwrap() is used to handle Result type that can give Ok() or Err()
            // if we get an Err after unwrap() the program will halt immediately
            // not considered good practice in prod as its best to handle err cases properly
            fs::create_dir_all(&outpath).unwrap();
        }else{
            // if the current file is a file
            println!("File {} extracted to {} ({} bytes)", i, outpath.display(), file.size());
            // check if file has parent
            if let Some(p) = outpath.parent(){
                if !p.exists(){
                    //if parent does not exist create the file here
                    fs::create_dir_all(&p).unwrap();
                }
            }
            //actually creating the unzipped file (folder)
            let mut outfile = fs::File::create(&outpath).unwrap();
            //copy the file read in the zip folder to the unzipped folder
            io::copy(&mut file, &mut outfile).unwrap();
        }
        //set permisions
        #[cfg(unix)] // this line is used for conditional compilation (if in unix OS this will compile)
        {
            use std::os::unix::fs::PermissionsExt; // Import Unix-specific file permissions extensions
            
            if let Some(mode) = file.unix_mode(){
                // Try to get the Unix mode (permissions) of the file
                // If the mode is available, set the permissions of the outpath to match the file's permissions
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    return 0;
}
