use std::fmt;

#[derive(PartialEq)]
pub struct Track {
    pub name: String,
    pub duration: u64,
}
impl fmt::Debug for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Track")
            .field("name", &self.name)
            .field("duration", &self.duration)
            .finish()
    }
}