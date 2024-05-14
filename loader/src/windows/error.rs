/// An error has occurred while loading the drivers
#[derive(Debug)]
pub enum LoadError {}

impl std::error::Error for LoadError {}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
