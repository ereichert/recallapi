pub struct Memory<'a> {
    pub id: Option<usize>,
    pub prompt: &'a str,
    pub details: &'a str,
}

impl<'a> Memory<'a> {
    pub fn new(prompt: &'a str, details: &'a str) -> Memory<'a> {
        Memory {
            id: None,
            prompt,
            details,
        }
    }
}