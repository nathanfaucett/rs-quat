use num::Num;
use create::{clone, create};


#[inline(always)]
pub fn rotation_x<'a, 'b, T: Num>(out: &'b [T; 4]) -> T {
    (T::from_isize(2isize) * out[0] * out[3] + T::from_isize(2isize) * out[1] * out[2])
        .atan2(T::one() - T::from_isize(2isize) * (out[2] * out[2] + out[3] * out[3]))
}

#[inline(always)]
pub fn rotation_y<'a, 'b, T: Num>(out: &'b [T; 4]) -> T {
    let theta = T::from_isize(2isize) * (out[0] * out[2] + out[3] * out[1]);
    (if theta < -T::one() {-T::one()} else if theta > T::one() {T::one()} else {theta}).asin()
}

#[inline(always)]
pub fn rotation_z<'a, 'b, T: Num>(out: &'b [T; 4]) -> T {
    (T::from_isize(2isize) * out[0] * out[1] + T::from_isize(2isize) * out[2] * out[3])
        .atan2(T::one() - T::from_isize(2isize) * (out[1] * out[1] + out[2] * out[2]))
}

#[inline(always)]
pub fn rotate_x<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], angle: T) -> &'a mut [T; 4] {
    let ax = a[0];
    let ay = a[1];
    let az = a[2];
    let aw = a[3];
    let half_angle = angle / T::from_isize(2isize);
    let bx = half_angle.sin();
    let bw = half_angle.cos();

    out[0] = ax * bw + aw * bx;
    out[1] = ay * bw + az * bx;
    out[2] = az * bw - ay * bx;
    out[3] = aw * bw - ax * bx;
    out
}

#[inline(always)]
pub fn rotate_y<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], angle: T) -> &'a mut [T; 4] {
    let ax = a[0];
    let ay = a[1];
    let az = a[2];
    let aw = a[3];
    let half_angle = angle / T::from_isize(2isize);
    let by = half_angle.sin();
    let bw = half_angle.cos();

    out[0] = ax * bw - az * by;
    out[1] = ay * bw + aw * by;
    out[2] = az * bw + ax * by;
    out[3] = aw * bw - ay * by;
    out
}

#[inline(always)]
pub fn rotate_z<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], angle: T) -> &'a mut [T; 4] {
    let ax = a[0];
    let ay = a[1];
    let az = a[2];
    let aw = a[3];
    let half_angle = angle / T::from_isize(2isize);
    let bz = half_angle.sin();
    let bw = half_angle.cos();

    out[0] = ax * bw + ay * bz;
    out[1] = ay * bw - ax * bz;
    out[2] = az * bw + aw * bz;
    out[3] = aw * bw - az * bz;
    out
}

#[inline(always)]
pub fn rotate<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], x: T, y: T, z: T) -> &'a mut [T; 4] {
    let mut tmp_a = clone(a);
    let mut tmp_b = create(T::zero(), T::zero(), T::zero(), T::one());
    rotate_z(&mut tmp_a, &a, z);
    rotate_x(&mut tmp_b, &tmp_a, x);
    rotate_y(out, &tmp_b, y);
    out
}

#[inline(always)]
pub fn look_rotation<'a, 'b, T: Num>(out: &'a mut [T; 4], forward: [T; 3], up: [T; 3]) -> &'a mut [T; 4] {
    let fx = forward[0];
    let fy = forward[1];
    let fz = forward[2];
    let ux = up[0];
    let uy = up[1];
    let uz = up[2];

    let ax = uy * fz - uz * fy;
    let ay = uz * fx - ux * fz;
    let az = ux * fy - uy * fx;

    let d = (T::one() + ux * fx + uy * fy + uz * fz) * T::from_isize(2isize);
    let dsq = d * d;
    let s = if dsq != T::zero() {T::one() / dsq} else {dsq};

    out[0] = ax * s;
    out[1] = ay * s;
    out[2] = az * s;
    out[3] = dsq / T::from_isize(2isize);
    out
}

#[inline(always)]
pub fn from_axis_angle<'a, 'b, T: Num>(out: &'a mut [T; 4], axis: [T; 3], angle: T) -> &'a mut [T; 4] {
    let half_angle = angle / T::from_isize(2isize);
    let s = half_angle.sin();

    out[0] = axis[0] * s;
    out[1] = axis[1] * s;
    out[2] = axis[2] * s;
    out[3] = half_angle.cos();
    out
}

#[inline(always)]
pub fn from_mat<'a, 'b, T: Num>(
    out: &'a mut [T; 4],
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T
) -> &'a mut [T; 4] {
    let trace = m11 + m22 + m33;

    if trace > T::zero() {
        let s = T::from_f32(0.5f32) / (trace + T::one()).sqrt();

        out[3] = T::from_f32(0.25f32) / s;
        out[0] = (m32 - m23) * s;
        out[1] = (m13 - m31) * s;
        out[2] = (m21 - m12) * s;
    } else if m11 > m22 && m11 > m33 {
        let s = T::from_isize(2isize) * (T::one() + m11 - m22 - m33).sqrt();
        let inv_s = T::one() / s;

        out[3] = (m32 - m23) * inv_s;
        out[0] = s / T::from_isize(4isize);
        out[1] = (m12 + m21) * inv_s;
        out[2] = (m13 + m31) * inv_s;
    } else if m22 > m33 {
        let s = T::from_isize(2isize) * (T::one() + m22 - m11 - m33).sqrt();
        let inv_s = T::one() / s;

        out[3] = (m13 - m31) * inv_s;
        out[0] = (m12 + m21) * inv_s;
        out[1] = s / T::from_isize(4isize);
        out[2] = (m23 + m32) * inv_s;
    } else {
        let s = T::from_isize(2isize) * (T::one() + m33 - m11 - m22).sqrt();
        let inv_s = T::one() / s;

        out[3] = (m21 - m12) * inv_s;
        out[0] = (m13 + m31) * inv_s;
        out[1] = (m23 + m32) * inv_s;
        out[2] = s / T::from_isize(4isize);
    }
    out
}

#[inline(always)]
pub fn from_mat2<'a, 'b, T: Num>(out: &'a mut [T; 4], m: &'b [T; 4]) -> &'a mut [T; 4] {
    from_mat(
        out,
        m[0], m[2], T::zero(),
        m[1], m[3], T::zero(),
        T::zero(), T::zero(), T::one()
    )
}

#[inline(always)]
pub fn from_mat32<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 6]) -> &'a mut [T; 4] {
    from_mat(
        out,
        m[0], m[2], T::zero(),
        m[1], m[3], T::zero(),
        T::zero(), T::zero(), T::one()
    )
}

#[inline(always)]
pub fn from_mat3<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 9]) -> &'a mut [T; 4] {
    from_mat(
        out,
        m[0], m[3], m[6],
        m[1], m[4], m[7],
        m[2], m[5], m[8]
    )
}

#[inline(always)]
pub fn from_mat4<'a, 'b, T: Num>(out: &'a mut [T; 4], m: [T; 16]) -> &'a mut [T; 4] {
    from_mat(
        out,
        m[0], m[4], m[8],
        m[1], m[5], m[9],
        m[2], m[6], m[10]
    )
}
