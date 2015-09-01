use num::Num;
use create::clone;
use length::{normalize, dot};

#[inline(always)]
pub fn inverse<T: Num>(out: &mut [T; 4], a: [T; 4]) -> &mut [T; 4] {
    let mut d = dot(a, a);

    if d != T::zero() {
        d = T::one() / d
    }

    out[0] = a[0] * -d;
    out[1] = a[1] * -d;
    out[2] = a[2] * -d;
    out[3] = a[3] * -d;
    out
}
#[test]
fn test_inverse() {
    let mut v = [0.0, 0.0, 0.0, 0.0];
    inverse(&mut v, [1.0, 1.0, 1.0, 1.0]);
    assert!(v[0] == -0.25);
    assert!(v[1] == -0.25);
    assert!(v[2] == -0.25);
    assert!(v[3] == -0.25);
}

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
pub fn lerp<T: Num>(out: &mut [T; 4], a: [T; 4], b: [T; 4], t: T) -> &mut [T; 4] {
    out[0] = a[0] + (b[0] - a[0]) * t;
    out[1] = a[1] + (b[1] - a[1]) * t;
    out[2] = a[2] + (b[2] - a[2]) * t;
    out[3] = a[3] + (b[3] - a[3]) * t;
    out
}
#[test]
fn test_lerp() {
    let mut v = [0.0, 0.0, 0.0, 0.0];
    lerp(&mut v, [0.0, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0], 0.5);
    assert!(v[0] == 0.5);
    assert!(v[1] == 0.5);
    assert!(v[2] == 0.5);
    assert!(v[3] == 0.5);
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

#[inline(always)]
pub fn min<T: Num>(out: &mut [T; 4], a: [T; 4], b: [T; 4]) -> &mut [T; 4] {
    out[0] = if b[0] < a[0] {b[0]} else {a[0]};
    out[1] = if b[1] < a[1] {b[1]} else {a[1]};
    out[2] = if b[2] < a[2] {b[2]} else {a[2]};
    out[3] = if b[3] < a[3] {b[3]} else {a[3]};
    out
}
#[test]
fn test_min() {
    let mut v = [0, 0, 0, 0];
    min(&mut v, [1, 0, 1, 0], [0, 1, 0, 1]);
    assert!(v == [0, 0, 0, 0]);
}

#[inline(always)]
pub fn max<T: Num>(out: &mut [T; 4], a: [T; 4], b: [T; 4]) -> &mut [T; 4] {
    out[0] = if b[0] > a[0] {b[0]} else {a[0]};
    out[1] = if b[1] > a[1] {b[1]} else {a[1]};
    out[2] = if b[2] > a[2] {b[2]} else {a[2]};
    out[3] = if b[3] > a[3] {b[3]} else {a[3]};
    out
}
#[test]
fn test_max() {
    let mut v = [0, 0, 0, 0];
    max(&mut v, [1, 0, 1, 0], [0, 1, 0, 1]);
    assert!(v == [1, 1, 1, 1]);
}

#[inline(always)]
pub fn clamp<T: Num>(out: &mut [T; 4], a: [T; 4], min: [T; 4], max: [T; 4]) -> &mut [T; 4] {
    out[0] = if a[0] < min[0] {min[0]} else if a[0] > max[0] {max[0]} else {a[0]};
    out[1] = if a[1] < min[1] {min[1]} else if a[1] > max[1] {max[1]} else {a[1]};
    out[2] = if a[2] < min[2] {min[2]} else if a[2] > max[2] {max[2]} else {a[2]};
    out[3] = if a[3] < min[3] {min[3]} else if a[3] > max[3] {max[3]} else {a[3]};
    out
}
#[test]
fn test_clamp() {
    let mut v = [0, 0, 0, 0];
    clamp(&mut v, [2, 2, 2, 2], [0, 0, 0, 0], [1, 1, 1, 1]);
    assert!(v == [1, 1, 1, 1]);
}
