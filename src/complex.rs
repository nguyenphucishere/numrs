use std::ops::{Add, Sub, Mul, Div};

use crate::utils::numbers::Numeric;

struct Complex<N: Numeric>{
    re: N,
    im: N,
}

impl<N: Numeric> Complex<N> {
    pub fn new(re: N, im: N) -> Self {
        Self { re, im }
    }

    pub fn Re(&self) -> N {
        self.re
    }

    pub fn Im(&self) -> N {
        self.im
    }

    pub fn Conj(&self) -> Self {
        Self {
            re: self.re,
            im: N::negative() * self.im,
        }
    }

    pub fn sq_norm(&self) -> N {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(&self) -> N {
        self.sq_norm().sqrt()
    }

    pub fn arg(&self) -> N {
        N::from_float(self.im.to_float().atan2(self.re.to_float()))
    }


    pub fn pow(&self, n: usize) -> Self {
        let r = self.norm();
        let theta = self.arg();
        let r_n = N::to_float(r).powi(n as i32);
        let theta_n = theta * N::from_float(n as f64);
        Self {
            re: N::from_float(r_n * theta_n.to_float().cos()),
            im: N::from_float(r_n * theta_n.to_float().sin()),
        }
    }

    pub fn exp(&self) -> Self {
        let exp_re = N::from_float(N::to_float(self.re).exp());
        let cos_im = N::from_float(N::to_float(self.im).cos());
        let sin_im = N::from_float(N::to_float(self.im).sin());
        Self {
            re: exp_re * cos_im,
            im: exp_re * sin_im,
        }
    }

    pub fn ln(&self) -> Self {
        let r = self.norm();
        let theta = self.arg();
        Self {
            re: N::from_float(N::to_float(r).ln()),
            im: theta,
        }
    }

    pub fn sin(&self) -> Self {
        let sin_re = N::from_float(N::to_float(self.re).sin());
        let cos_re = N::from_float(N::to_float(self.re).cos());
        let sinh_im = N::from_float(N::to_float(self.im).sinh());
        let cosh_im = N::from_float(N::to_float(self.im).cosh());
        Self {
            re: sin_re * cosh_im,
            im: cos_re * sinh_im,
        }
    }

    pub fn cos(&self) -> Self {
        let cos_re = N::from_float(N::to_float(self.re).cos());
        let sin_re = N::from_float(N::to_float(self.re).sin());
        let cosh_im = N::from_float(N::to_float(self.im).cosh());
        let sinh_im = N::from_float(N::to_float(self.im).sinh());
        Self {
            re: cos_re * cosh_im,
            im: N::negative() * sin_re * sinh_im,
        }
    }

    pub fn tan(&self) -> Self {
        let sin_z = self.sin();
        let cos_z = self.cos();
        sin_z / cos_z
    }

    pub fn from_polar(r: N, theta: N) -> Self {
        Self {
            re: N::from_float(N::to_float(r) * N::to_float(theta).cos()),
            im: N::from_float(N::to_float(r) * N::to_float(theta).sin()),
        }
    }

    pub fn to_polar(&self) -> (N, N) {
        (self.norm(), self.arg())
    }

}

impl<N: Numeric> Add for Complex<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl<N: Numeric> Sub for Complex<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re + N::negative() * other.re,
            im: self.im + N::negative() * other.im,
        }
    }
}

impl<N: Numeric> Mul for Complex<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re + N::negative() * self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl<N: Numeric> Div for Complex<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.sq_norm();
        let re = (self.re * other.re + self.im * other.im) / denom;
        let im = (self.im * other.re + N::negative() * self.re * other.im) / denom;
        Self { re, im }
    }
}