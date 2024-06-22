use std::fs;
use std::io;

fn main() {
    std::process::exit(decompressor());
}

fn decompressor() -> i32 {
    let args: Vec<_> = std::env::args().collect(); //Vector is used to store args provided by the
                                                   //user

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]); //min 2 args (name of file check)
        return 1;
    }

    let f_name = std::path::Path::new(&*args[1]); //path of the file

    let file = fs::File::open(&f_name).unwrap(); //opening file

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        //if many files are there in zip container then this will
        //take are of it
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            //multiple folders can also esist
            println!("File {} extracted to \"{}\"", i, outpath.display());

            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)] //Permissions for extracting files
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}
