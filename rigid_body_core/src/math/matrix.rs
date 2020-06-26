#![allow(dead_code)]
use std::{
    default::Default,
    fmt::{
	self,
	Display,
	Formatter,
    },
    f64::EPSILON,
    ops::{
	Index,
	IndexMut,
    },
};

macro_rules! get_mat_value {
    ($mat:ident, $k:ident, $j:ident, nottranspose) => {
	$mat[$k][$j];
    };
    
    ($mat:ident, $k:ident, $j:ident, transpose) => {
	$mat[$j][$k];
    };
}

macro_rules! gen_mult_fn {
    ($name:ident, $matrix:ident) => {
	gen_mult_fn!($name, $matrix, nottranspose);
    };
    
    ($name:ident, $matrix:ident, $opt:ident) => {
	#[must_use]
	pub fn $name(&self, other: &$matrix) -> $matrix {
	    let mut ret = Self::default();
	    let size = self.m.len();
	    for i in 0..size {
		for j in 0..size {
		    for k in 0..size {
			ret[i][j] += self[i][k]*get_mat_value!(other, k, j, $opt);
		    }
		}
	    }
	    ret
	}
    };
}

macro_rules! gen_matrix {
    ($matrix:ident, $n:expr) => {
	#[derive(Clone, Copy, Debug)]
	pub struct $matrix {
	    m: [[f64; $n]; $n]
	}

	impl $matrix {
	    pub fn new(m: &[[f64; $n]; $n]) -> Self {
		Self{m: *m}
	    }

	    pub fn identity() -> Self {
		let mut ret = Self::default();
		for i in 0..$n {
		    ret[i][i] = 1.;
		}
		ret
	    }

	    pub fn add(&self, other: &Self) -> Self {
		let mut ret = *self;
		ret.add_assign(other);
		ret
	    }
	    
	    pub fn add_assign(&mut self, other: &Self) {
		for i in 0..$n {
		    for j in 0..$n {
			self.m[i][j] += other[i][j];
		    }
		}
	    }

	    pub fn is_zero(&self) -> bool {
		for i in 0..$n {
		    for j in 0..$n {
			if self.m[i][j] >= EPSILON {
			    return false;
			}
		    }
		}
		true
	    }
	    
	    gen_mult_fn!(mult, $matrix);
	    gen_mult_fn!(mult_t, $matrix, transpose);

	    pub fn scale(&self, factor: f64) -> Self {
		let mut ret = *self;
		ret.scale_assign(factor);
		ret
	    }
	    
	    pub fn scale_assign(&mut self, factor: f64) {
		for i in 0..$n {
		    for j in 0..$n {
			self.m[i][j] *= factor;
		    }
		}
	    }

	    pub fn trace(&self) -> f64 {
		let mut ret = 0.;
		for i in 0..$n {
		    ret += self.m[i][i];
		}
		ret
	    }
	}
	
	impl Default for $matrix {
	    #[must_use]
	    fn default() -> Self {
		Self{m: [[0.; $n]; $n]}
	    }
	}

	impl Display for $matrix {
	    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		for i in 0..$n {
		    write!(f, "{:?},\n", self.m[i])?;
		}
		Ok(())
	    }
	}
	
	impl Index<usize> for $matrix {
	    type Output = [f64; $n];
	    #[must_use]
	    fn index(&self, index: usize) -> &Self::Output {
		&self.m[index]
	    }
	}

	impl IndexMut<usize> for $matrix {
	    #[must_use]
	    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.m[index]
	    }
	}
    }
}

gen_matrix!(Matrix2x2, 2);
gen_matrix!(Matrix3x3, 3);
gen_matrix!(Matrix4x4, 4);

impl Matrix2x2 {
    pub fn det(&self) -> f64 {
	self.m[0][0]*self.m[1][1]-self.m[0][1]*self.m[1][0]
    }
}

impl Matrix3x3 {
    pub fn inverse(&self) -> Option<Self> {
	let mut ret = Self::default();
	for i in 0..3 {
	    for j in 0..3 {
		ret[j][i] =
		    if (i+j)%2 == 0 {1.} else {-1.} * self.minor(i, j).det();
	    }
	}
	let det = self.m[0][0]*ret[0][0]
	    +self.m[0][1]*ret[1][0]
	    +self.m[0][2]*ret[2][0];
	if det.abs() < EPSILON {None}
	else {
	    ret.scale_assign(1./det);
	    Some(ret)
	}
    }
    
    fn minor(&self, i: usize, j: usize) -> Matrix2x2 {
	let mut ret = Matrix2x2::default();
	let mut k = 0;
	let mut k_minor = 0;
	while k < 3 {
	    if k != i {
		let mut l = 0;
		let mut l_minor = 0;
		while l < 3 {
		    if l != j {
			ret[k_minor][l_minor] = self.m[k][l];
			l_minor += 1;
		    }
		    l += 1;
		}
		k_minor += 1;
	    }
	    k += 1;
	}
	ret
    }

}
