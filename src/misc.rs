use num::Num;
use signed::Signed;
use create::clone;
use length::normalize;


pub use vec4::inverse;
pub use vec4::lerp;
pub use vec4::min;
pub use vec4::max;
pub use vec4::clamp;
pub use vec4::eq;
pub use vec4::ne;


#[inline]
pub fn conjugate<'a, 'b, T: Signed>(out: &'a mut [T; 4], a: &'b [T; 4]) -> &'a mut [T; 4] {
    out[0] = -a[0];
    out[1] = -a[1];
    out[2] = -a[2];
    out[3] = a[3];
    out
}
#[test]
fn test_conjugate() {
    let mut v = [0, 0, 0, 0];
    conjugate(&mut v, &[1, 1, 1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == -1);
    assert!(v[2] == -1);
    assert!(v[3] == 1);
}

#[inline]
pub fn calculate_w<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4]) -> &'a mut [T; 4] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = (T::one() - a[0] * a[0] - a[1] * a[1] - a[2] * a[2]).abs().sqrt();
    out
}

#[inline]
pub fn nlerp<'a, 'b, T: Num, N: Num>(out: &'a mut [T; 4], a: &'b [T; 4], b: &'b [T; 4], t: N) -> &'a mut [T; 4] {
    let tmp = clone(lerp(out, a, b, t));
    normalize(out, &tmp)
}

#[inline]
pub fn slerp<'a, 'b, T: Signed, N: Signed>(out: &'a mut [T; 4], a: &'b [T; 4], b: &'b [T; 4], t: N) -> &'a mut [T; 4] {
    let t_f64 = t.to_f64();
    let ax = a[0].to_f64();
    let ay = a[1].to_f64();
    let az = a[2].to_f64();
    let aw = a[3].to_f64();
    let mut bx = b[0].to_f64();
    let mut by = b[1].to_f64();
    let mut bz = b[2].to_f64();
    let mut bw = b[3].to_f64();

    let mut cosom = ax * bx + ay * by + az * bz + aw * bw;
    let mut sinom;
    let omega;
    let scale0;
    let scale1;

    if cosom < 0_f64 {
        cosom = -cosom;
        bx = -bx;
        by = -by;
        bz = -bz;
        bw = -bw;
    }

    if 1_f64 - cosom > 0_f64 {
        omega = cosom.acos();

        sinom = omega.sin();
        sinom = if sinom != 0_f64 {1_f64 / sinom} else {sinom};

        scale0 = ((1_f64 - t_f64) * omega).sin() * sinom;
        scale1 = (t_f64 * omega).sin() * sinom;
    } else {
        scale0 = 1_f64 - t_f64;
        scale1 = t_f64;
    }

    out[0] = T::from_f64(scale0 * ax + scale1 * bx);
    out[1] = T::from_f64(scale0 * ay + scale1 * by);
    out[2] = T::from_f64(scale0 * az + scale1 * bz);
    out[3] = T::from_f64(scale0 * aw + scale1 * bw);

    out
}
