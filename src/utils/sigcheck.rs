use std::{fs::File, io};

///checks the signature of the downloaded file (currently hardcoded)
pub(crate) fn sigcheck(path: &str, signature: &str) -> io::Result<()> {
    let mut file = File::open(path).unwrap();
    let mut hasher = md5::Context::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.compute();
    let hash = format!("{:x}", hash);

    if hash.to_lowercase() == signature.to_lowercase() {
        println!(
            "Hash of the downloaded file: {} matches the expected hash: {}",
            path, signature
        );
        Ok(())
    } else {
        println!(
            "Hash of the downloaded file: {} does not match the expected hash: {}",
            path, signature
        );
        println!("\nDo you want to continue? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            println!("Continuing...");
            Ok(())
        } else {
            println!("Exiting...");
            std::process::exit(0);
        }
    }
}

