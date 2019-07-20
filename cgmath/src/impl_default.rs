use std::default::Default;

use num::BaseFloat;
use num_traits::One;

use vector::*;
use point::*;
use matrix::*;

impl<S: Default> Default for Vector1<S> {
    fn default() -> Self{
        Vector1::new(S::default())
    }
}

impl<S: Default> Default for Vector2<S> {
    fn default() -> Self{
        Vector2::new(S::default(), S::default())
    }
}

impl<S: Default> Default for Vector3<S> {
    fn default() -> Self{
        Vector3::new(S::default(), S::default(), S::default())
    }
}

impl<S: Default> Default for Vector4<S> {
    fn default() -> Self{
        Vector4::new(S::default(), S::default(), S::default(), S::default())
    }
}

impl<S: Default> Default for Point1<S> {
    fn default() -> Self{
        Point1::new(S::default())
    }
}

impl<S: Default> Default for Point2<S> {
    fn default() -> Self{
        Point2::new(S::default(), S::default())
    }
}

impl<S: Default> Default for Point3<S> {
    fn default() -> Self{
        Point3::new(S::default(), S::default(), S::default())
    }
}

impl<S: BaseFloat> Default for Matrix2<S> {
    fn default() -> Self{
        Matrix2::one()
    }
}

impl<S: BaseFloat> Default for Matrix3<S> {
    fn default() -> Self{
        Matrix3::one()
    }
}

impl<S: BaseFloat> Default for Matrix4<S> {
    fn default() -> Self{
        Matrix4::one()
    }
}