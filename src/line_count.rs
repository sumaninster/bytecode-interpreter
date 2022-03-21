pub mod line_counter {
    use std::fs;
    use std::fs::{metadata, File};
    use std::io::{BufReader, BufRead};
    use std::ffi::OsStr;
    use crate::{output_ln};
    /*
    Function to count number of lines in a file with given extension
     */
    #[allow(dead_code)]
    pub fn count_lines(dir: &str, ext: &str) {
        let paths = fs::read_dir(dir).unwrap();
        for path in paths {
            match path {
                Ok(path) => {
                    let md = metadata(path.path()).unwrap();
                    if md.is_dir() {
                        count_lines(path.path().as_os_str().to_str().unwrap(), ext);
                    } else if md.is_file() {
                        let s = path.file_name();
                        let file_path = std::path::Path::new(s.as_os_str());
                        if file_path.extension().and_then(OsStr::to_str) == Some(ext) {
                            output_ln!(format!("{}: {}", path.path().display(),
                                         BufReader::new(File::open(path.path()).unwrap()).lines().count()));
                        }
                    }
                },
                Err(e) => output_ln!(format!("{:?}", e)),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::line_count::line_counter::count_lines;
    #[test]
    fn count_lines_for_files_with_bc_extension() {
        count_lines("./", "bc");
    }
    #[test]
    fn count_lines_for_files_with_rs_extension() {
        count_lines("./", "rs");
    }
}