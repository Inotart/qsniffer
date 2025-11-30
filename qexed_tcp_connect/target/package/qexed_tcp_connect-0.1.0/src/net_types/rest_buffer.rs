#[derive(Debug, Default, PartialEq,Clone)]
pub struct RestBuffer(pub Vec<u8>);
impl RestBuffer{
    pub fn new() -> Self {
        RestBuffer(Vec::new())
    }
}