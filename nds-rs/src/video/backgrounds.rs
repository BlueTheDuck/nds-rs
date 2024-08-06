use super::AffineTransform;

mod bitmap;
pub use bitmap::*;

/// Marker trait for backgrounds that support affine transformations
pub trait AffineBackgroundMarker: crate::private::Sealed {
    const AFFINE_MATRIX_REGISTER: *mut AffineTransform;

    fn set_transformation(&self, transformation: &AffineTransform) {
        unsafe {
            Self::AFFINE_MATRIX_REGISTER.write_volatile(*transformation);
        }
    }
    fn transformation(&self) -> AffineTransform {
        unsafe { Self::AFFINE_MATRIX_REGISTER.read_volatile() }
    }
}
