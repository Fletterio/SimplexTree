use std::cmp::PartialOrd;

#[derive(PartialEq, PartialOrd, Debug)]
//should I use const generics?
pub struct Point {
    pub coordinates: Vec<f64>,
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
