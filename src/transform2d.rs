
/// Implemented by contexts that can transform.
pub trait Transform2d<'a> {
    /// Translate x and y.
    fn trans(&'a self, x: f64, y: f64) -> Self;
   
    /// Rotates degrees.
    #[inline(always)]
    fn rot_deg(&'a self, angle: f64) -> Self {
        let pi: f64 = Float::pi();
        self.rot_rad(angle * pi / 180.0)
    }
    
    /// Rotate radians.
    fn rot_rad(&'a self, angle: f64) -> Self;
    /// Scale.
    fn scale(&'a self, sx: f64, sy: f64) -> Self;
    /// Shear.
    fn shear(&'a self, sx: f64, sy: f64) -> Self;
}