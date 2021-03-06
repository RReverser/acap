//! [`$\ell^p$`]/[Minkowski] distance.
//!
//! [`$\ell^p$`]: https://en.wikipedia.org/wiki/Lp_space
//! [Minkowski]: https://en.wikipedia.org/wiki/Minkowski_distance

use crate::coords::Coordinates;
use crate::distance::Proximity;

use num_traits::real::Real;
use num_traits::zero;

/// A point in L<sup>1</sup> space.
pub use crate::taxi::Taxicab as L1;

/// Compute the L<sup>1</sup> distance between two points.
pub use crate::taxi::taxicab_distance as l1_distance;

/// A point in L<sup>2</sup> space.
pub use crate::euclid::Euclidean as L2;
/// An L<sup>2</sup> distance.
pub use crate::euclid::EuclideanDistance as L2Distance;

/// Compute the L<sup>2</sup> distance between two points.
pub use crate::euclid::euclidean_distance as l2_distance;

/// A point in L<sup>∞</sup> space.
pub use crate::chebyshev::Chebyshev as Linf;

/// Compute the L<sup>∞</sup> distance between two points.
pub use crate::chebyshev::chebyshev_distance as linf_distance;

/// Compute the [`$\ell^p$`]/[Minkowski] distance between two points.
///
/// ```math
/// \begin{aligned}
/// \mathrm{lp\_distance}(p, x, y) &= \|x - y\|_p \\
/// &= \left( \sum_i |x_i - y_i|^p \right)^{1/p}
/// \end{aligned}
/// ```
///
/// [`$\ell^p$`]: https://en.wikipedia.org/wiki/Lp_space
/// [Minkowski]: https://en.wikipedia.org/wiki/Minkowski_distance
pub fn lp_distance<T, U>(p: T::Value, x: T, y: U) -> T::Value
where
    T: Coordinates,
    U: Coordinates<Value = T::Value>,
    T::Value: Real,
{
    debug_assert!(x.dims() == y.dims());

    let mut sum: T::Value = zero();
    for i in 0..x.dims() {
        sum += (x.coord(i) - y.coord(i)).abs().powf(p);
    }

    sum.powf(p.recip())
}

/// Marker trait for [Minkowski distances].
///
/// [Minkowski distances]: https://en.wikipedia.org/wiki/Minkowski_distance
pub trait Minkowski<T: ?Sized = Self>: Proximity<T> {}

/// Blanket [`Minkowski`] implementation for references.
impl<'k, 'v, K: Minkowski<V>, V> Minkowski<&'v V> for &'k K {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lp_distance() {
        assert_eq!(l1_distance(&[0.0, 0.0], &[3.0, 4.0]), 7.0);
        assert_eq!(l2_distance(&[0.0, 0.0], &[3.0, 4.0]), 5.0);
        assert!(lp_distance(3.0, &[0.0, 0.0], &[3.0, 4.0]) < 5.0);
        assert_eq!(linf_distance(&[0.0, 0.0], &[3.0, 4.0]), 4.0);
    }
}
