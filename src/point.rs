use std::cmp::PartialOrd;

#[derive(PartialEq, PartialOrd)]
//should I use const generics?
pub struct Point {
    coordinates: Vec<f64>,
}
