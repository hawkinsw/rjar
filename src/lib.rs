#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Jar {
}

impl Jar {
    pub fn open(jar_filename: String) -> Result<Self, &'static str> {
        Ok(Self{})
    }

    pub fn file_contents_by_name(jar_member_filename: String) -> Vec<u8> {
        vec![]
    }
}
