
struct PixelRGB16 {
    
}

struct PixelBW {

}

#[cfg(RGB16)]
pub type Pixel = PixelRGB16;

#[cfg(not(RGB))]
pub type Pixel = PixelBW;

