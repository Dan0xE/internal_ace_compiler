use std::{fs::File, io};

///checks the signature of the downloaded file (currently hardcoded)
pub(crate) fn sigcheck(path: &str, signature: &str) -> bool {
    let mut file = File::open(path).unwrap();
    let mut hasher = md5::Context::new();
    io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.compute();
    let hash = format!("{:x}", hash);
    // we might change the behavior of this function in the future to not return a bool but instead handle it common to all check by exiting the program if the check fails
    if hash.to_lowercase() == signature.to_lowercase() {
        true
    } else {
        false
    }
}
