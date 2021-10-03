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
impl<T: PartialEq> Eq for Complex<T> {}
