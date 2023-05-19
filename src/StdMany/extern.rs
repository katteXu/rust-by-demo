use core::fmt;

#[link(name = "m")]
extern "C" {
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0f32 {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}", self.re, self.im)
        }
    }
}

fn main() {
    let z = Complex {
        re: -1f32,
        im: 0f32,
    };

    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    println!("cos({:?} = {:?})", z, cos(z));
}
