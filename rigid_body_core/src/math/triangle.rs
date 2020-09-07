use super::vector::{Vector2d, Vector3d, Vector4d};

const NVERTICES: usize = 3;

macro_rules! gen_triangle {
    ($triangle:ident, $vector:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $triangle {
            pub vertices: [$vector; NVERTICES],
        }

        impl $triangle {
            #[must_use]
            pub fn new(a: &$vector, b: &$vector, c: &$vector) -> Self {
                Self {
                    vertices: [*a, *b, *c],
                }
            }
        }
    };
}

gen_triangle!(Triangle2d, Vector2d);
gen_triangle!(Triangle3d, Vector3d);
gen_triangle!(Triangle4d, Vector4d);
