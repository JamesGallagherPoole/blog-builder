pub struct Paths {
    pub input: String,
    pub output: String,
}

impl Paths {
    pub fn new() -> Paths {
        Paths {
            input: String::new(),
            output: String::new(),
        }
    }
}
