use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    pub fn new(name: &str, data: Vec<u8>) -> File {
        File {
            name: String::from(name),
            data,
            state: FileState::Closed,
        }
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            let err_msg = String::from("File must be open for reading");
            return Err(err_msg);
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }

    pub fn open(mut self: File) -> Result<File, String> {
        if Self::one_in(0.1) {
            let err_msg = String::from("Permission denied");
            return Err(err_msg);
        }

        self.state = FileState::Open;

        Ok(self)
    }

    pub fn close(mut self: File) -> Result<File, String> {
        if Self::one_in(0.1) {
            let err_msg = String::from("Permission denied");
            return Err(err_msg);
        }

        self.state = FileState::Closed;

        Ok(self)
    }

    fn one_in(n: f64) -> bool {
        rand::thread_rng().gen_bool(n)
    }
}
