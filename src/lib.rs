
pub mod types;

pub mod layer;
pub mod graphics;
pub mod buffer;

#[cfg(feature = "sdl")]
pub mod native;

pub struct Gui {

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
