use crate::image_buffer::buffer::ImageBuffer;
use crate::util::GrayValue;


impl ImageBuffer<GrayValue> {
    pub fn threshold_mut(&mut self, min: GrayValue, max: GrayValue) -> &mut Self
    {
        for p in self.data().iter_mut() {
            if *p >= min && *p <= max {
                *p = 1 as GrayValue;
            } 
            else {
                *p = 0 as GrayValue;
            }
    }
        self
    }
}