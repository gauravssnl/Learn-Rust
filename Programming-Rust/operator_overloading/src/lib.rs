pub struct Complex<T> {
    re: T,
    im: T,
}

pub struct Complex1<T> {
    re: T,
    im: T,
}

use std::ops::Add;

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<L, R> Add<Complex1<R>> for Complex1<L>
where
    L: Add<R>,
{
    type Output = Complex1<L::Output>;
    fn add(self, rhs: Complex1<R>) -> Self::Output {
        Complex1 {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

use std::ops::Neg;

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

struct Boolean(bool);

use std::ops::Not;
impl Not for Boolean {
    type Output = Self;
    fn not(self) -> Self {
        match self.0 {
            true => Boolean(false),
            false => Boolean(true),
        }
    }
}

use std::ops::AddAssign;
impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

use std::cmp::PartialEq;

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Complex<T>) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
    // method ne() has default definition which uses eq()
}

use std::cmp::Eq;
impl<T: PartialEq + Iterator> Eq for Complex<T> {}

use std::cmp::Ordering;
use std::cmp::PartialOrd;

// Dummy example : Complex Numbers are not ordered
impl<T: PartialOrd> PartialOrd<Complex<T>> for Complex<T> {
    fn partial_cmp(&self, other: &Complex<T>) -> Option<Ordering> {
        match (
            self.re.partial_cmp(&other.re),
            self.im.partial_cmp(&other.im),
        ) {
            (Some(Ordering::Equal), Some(Ordering::Equal)) => Some(Ordering::Equal),
            (Some(Ordering::Equal), Some(Ordering::Greater)) => Some(Ordering::Greater),
            (Some(Ordering::Equal), Some(Ordering::Less)) => Some(Ordering::Less),
            (Some(Ordering::Greater), _) => Some(Ordering::Greater),
            (Some(Ordering::Less), _) => Some(Ordering::Less),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Interval<T> {
    pub lower: T, // inclusive
    pub upper: T, // exclusive
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.lower {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Student {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct StudentList {
    pub list: Vec<Student>,
}

use std::ops::{Index, IndexMut};

impl Index<usize> for StudentList {
    type Output = Student;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

// IndexMut<Indx> : Index<Indx>
impl IndexMut<usize> for StudentList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // both works.
        &mut self.list[index] // self.list.index_mut(index)
    }
}

// To enable image[row][column] access.

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    // Create a new image of the given size
    fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &Self::Output {
        let start = self.width * row;
        &self.pixels[start..start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    // &mut Self::Output also works as return type , since IndexMut<Indx> : Index<Index> where Self:Output is associated
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = self.width * row;
        &mut self.pixels[start..start + self.width]
    }
}
