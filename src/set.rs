use num::Num;


#[inline(always)]
pub fn set<T: Num>(out: &mut [T; 4], x: T, y: T, z: T, w: T) -> &mut [T; 4] {
    out[0] = x;
    out[1] = y;
    out[2] = z;
    out[3] = w;
    out
}
#[test]
fn test_set() {
    let mut v = [0, 0, 0, 0];
    set(&mut v, 1, 2, 3, 4);
    assert!(v == [1, 2, 3, 4]);
}
