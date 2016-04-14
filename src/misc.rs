use num::Num;
use create::clone;
use length::normalize;


pub use vec4::inverse;
pub use vec4::lerp;
pub use vec4::min;
pub use vec4::max;
pub use vec4::clamp;


#[inline(always)]
pub fn conjugate<T: Num>(out: &mut [T; 4], a: [T; 4]) -> &mut [T; 4] {
    out[0] = -a[0];
    out[1] = -a[1];
    out[2] = -a[2];
    out[3] = a[3];
    out
}
#[test]
fn test_conjugate() {
    let mut v = [0, 0, 0, 0];
    conjugate(&mut v, [1, 1, 1, 1]);
    assert!(v[0] == -1);
    assert!(v[1] == -1);
    assert!(v[2] == -1);
    assert!(v[3] == 1);
}

#[inline(always)]
pub fn calculate_w<T: Num>(out: &mut [T; 4], a: [T; 4]) -> &mut [T; 4] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = (T::one() - a[0] * a[0] - a[1] * a[1] - a[2] * a[2]).abs().sqrt();
    out
}

#[inline(always)]
pub fn nlerp<T: Num>(out: &mut [T; 4], a: [T; 4], b: [T; 4], t: T) -> &mut [T; 4] {
    let v = clone(*lerp(out, a, b, t));
    normalize(out, v)
}

#[inline(always)]
pub fn slerp<T: Num>(out: &mut [T; 4], a: [T; 4], b: [T; 4], t: T) -> &mut [T; 4] {
    let ax = a[0];
    let ay = a[1];
    let az = a[2];
    let aw = a[3];
    let mut bx = b[0];
    let mut by = b[1];
    let mut bz = b[2];
    let mut bw = b[3];

    let mut cosom = ax * bx + ay * by + az * bz + aw * bw;
    let mut sinom;
    let omega;
    let scale0;
    let scale1;

    if cosom < T::zero() {
        cosom = -cosom;
        bx = -bx;
        by = -by;
        bz = -bz;
        bw = -bw;
    }

    if T::one() - cosom > T::zero() {
        omega = cosom.acos();

        sinom = omega.sin();
        sinom = if sinom != T::zero() {T::one() / sinom} else {sinom};

        scale0 = ((T::one() - t) * omega).sin() * sinom;
        scale1 = (t * omega).sin() * sinom;
    } else {
        scale0 = T::one() - t;
        scale1 = t;
    }

    out[0] = scale0 * ax + scale1 * bx;
    out[1] = scale0 * ay + scale1 * by;
    out[2] = scale0 * az + scale1 * bz;
    out[3] = scale0 * aw + scale1 * bw;
    out
}
