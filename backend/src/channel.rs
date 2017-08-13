use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: u8,
    pub snowmix_id: u8,
    pub label: String,
    pub is_preview: bool,
    pub is_program: bool,
    pub is_dsk: bool
}

impl fmt::Debug for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Channel id: {}, snowmix_id: {}, label: {}>",
               self.id,
               self.snowmix_id,
               self.label)
    }
}
