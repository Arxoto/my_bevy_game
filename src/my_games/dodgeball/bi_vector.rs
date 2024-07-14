use std::ops;

pub type Unit = f64;

#[derive(Clone, Copy)]
pub struct BiVector(pub(crate) Unit, pub(crate) Unit);

impl ops::Add for BiVector {
    type Output = BiVector;

    fn add(self, rhs: Self) -> Self::Output {
        BiVector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for BiVector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl ops::Sub for BiVector {
    type Output = BiVector;

    fn sub(self, rhs: Self) -> Self::Output {
        BiVector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::SubAssign for BiVector {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl ops::Mul<Unit> for BiVector {
    type Output = BiVector;

    fn mul(self, rhs: Unit) -> Self::Output {
        BiVector(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Div<Unit> for BiVector {
    type Output = BiVector;

    fn div(self, rhs: Unit) -> Self::Output {
        BiVector(self.0 / rhs, self.1 / rhs)
    }
}
