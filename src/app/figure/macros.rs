macro_rules! impl_debuggable {
    ($ty:ty, $update_func:ident, $points:ident, $debug_offset:ident) => {
        impl Debuggable for $ty {
            fn update_frame(&mut self) -> bool {
                if let Some(pixels) = self.$update_func.next() {
                    self.$points.extend(pixels);
                    true
                } else {
                    false
                }
            }

            fn evaluate(&mut self) {
                while let Some(pixels) = self.$update_func.next() {
                    self.$points.extend(pixels);
                }
            }

            fn get_offset(&self) -> Pos2 {
                self.$debug_offset.clone()
            }

            fn get_pixels(&self) -> &[crate::pixel::Pixel] {
                self.$points.as_slice()
            }
        }
    };
}
