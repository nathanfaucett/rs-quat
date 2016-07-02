use num::Num;


#[inline(always)]
pub fn mul<'a, 'b, T: Num>(out: &'a mut [T; 4], a: &'b [T; 4], b: &'b [T; 4]) ->  &'a mut [T; 4] {
    out[0] = a[0] * -b[3] + a[3] * -b[0] + a[1] * -b[2] - a[2] * -b[1];
    out[1] = a[1] * -b[3] + a[3] * -b[1] + a[2] * -b[0] - a[0] * -b[2];
    out[2] = a[2] * -b[3] + a[3] * -b[2] + a[0] * -b[1] - a[1] * -b[0];
    out[3] = a[3] * -b[3] - a[0] * -b[0] - a[1] * -b[1] - a[2] * -b[2];
    out
}
