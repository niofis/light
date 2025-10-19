use light_types::world::World;
pub mod parsers;

pub fn load_folder(_path: &str) -> Option<World> {
    None
}

pub fn load_zip_file(_path: &str) -> Option<World> {
    None
}

pub fn load_zip_blob(_blob: &[u8]) -> Option<World> {
    None
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
