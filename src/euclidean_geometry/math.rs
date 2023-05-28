use crate::pbrt::*;

pub fn difference_of_products(a: Float, b: Float, c: Float, d: Float) -> Float {
    let cd = c * d;
    let difference_of_products = fma(a, b, -cd);
    let error = fma(-c, d, cd);
    return difference_of_products + error;
}
