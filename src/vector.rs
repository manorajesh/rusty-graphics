#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn rotate(&self, angle: f64) -> Vector<T>
    where
        T: Into<f64> + From<f64> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();

        let new_x = (x * angle.cos() - y * angle.sin()).into();
        let new_y = (x * angle.sin() + y * angle.cos()).into();

        Vector::new(new_x, new_y)
    }

    pub fn angle(&self) -> f64
    where
        T: Into<f64> + From<f64> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();
        y.atan2(x)
    }
}

impl<T> std::ops::Add for Vector<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> std::ops::AddAssign for Vector<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> std::ops::AddAssign<T> for Vector<T>
where
    T: std::ops::AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> std::ops::MulAssign for Vector<T>
where
    T: std::ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> std::ops::SubAssign for Vector<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> std::ops::SubAssign<T> for Vector<T>
where
    T: std::ops::SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T> std::ops::Mul<T> for Vector<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> std::ops::MulAssign<T> for Vector<T>
where
    T: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}