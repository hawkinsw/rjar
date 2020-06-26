extern crate zip;

use std::fs::File;
use std::io::Read;
use zip::read::ZipArchive;

pub struct Jar {
    archive: ZipArchive<File>,
}

impl Jar {
    pub fn open(jar_filename: &String) -> Result<Self, &'static str> {
        let file = File::open(jar_filename);
        // Can we open the file?
        if let Ok(file) = file {
            // Can we open the archive?
            let archive = ZipArchive::new(file);
            if let Ok(archive) = archive {
                return Ok(Self { archive });
            }
        }
        Err("Could not open the file!")
    }

    pub fn file_contents_by_name(
        &mut self,
        jar_member_filename: &String,
    ) -> Result<Vec<u8>, &'static str> {
        if let Ok(mut file) = self.archive.by_name(jar_member_filename) {
            let mut contents: Vec<u8> = vec![];
            if let Ok(_) = file.read_to_end(&mut contents) {
                return Ok(contents);
            }
        }
        Err("Could not read contents of requested file.")
    }

    pub fn file_names(&self) -> impl Iterator<Item = &str> {
        self.archive.file_names()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn open_existing_jar() {
        if let Ok(_) = super::Jar::open(&"testing.jar".to_string()) {
        } else {
            assert_eq!(true, false, "Failed to open existing Jar file.");
        }
    }

    #[test]
    fn open_nonexisting_jar() {
        if let Ok(_) = super::Jar::open(&"notfound.jar".to_string()) {
            assert_eq!(true, false, "Opened a non-existing Jar file.");
        }
    }

    #[test]
    fn list_files_of_existing_jar() {
        if let Ok(jar) = super::Jar::open(&"testing.jar".to_string()) {
            let filenames: Vec<&str> = jar.file_names().collect();
            assert_eq!(filenames.contains(&"t/java/lang/Class.class"), true);
            assert_eq!(filenames.contains(&"t/java/lang/Object.class"), true);
            assert_eq!(
                filenames.len(),
                5,
                "testing.jar does not contain the right number of files."
            );
        }
    }

    #[test]
    fn file_contents_of_existing_file_in_jar() {
        if let Ok(mut jar) = super::Jar::open(&"testing.jar".to_string()) {
            if let Ok(contents) = jar.file_contents_by_name(&"t/java/lang/Object.class".to_string())
            {
                let md5sum = md5::compute(contents);
                assert_eq!(
                    "5916745820b5eb3e5647da3b6cc6ef65",
                    format!("{:x}", md5sum),
                    "Bad file contents in testing zip file."
                );
                return;
            }
        }
        assert_eq!(
            true, false,
            "Could not read the contents of t/java/lang/Oject.class"
        );
    }

    #[test]
    fn file_contents_of_nonexisting_file_in_jar() {
        if let Ok(mut jar) = super::Jar::open(&"testing.jar".to_string()) {
            if let Err(_) = jar.file_contents_by_name(&"t/java/lang/System.class".to_string()) {
                return;
            }
        }
        assert_eq!(
            true, false,
            "Must not be able to read the contents of t/java/lang/System.class"
        );
    }
}
