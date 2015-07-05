use num::Num;


#[inline(always)]
pub fn create<T: Num>(x: T, y: T, z: T, w: T) -> [T; 4] {[x, y, z, w]}
#[test]
fn test_create() {
    let v = create(1, 2, 3, 4);
    assert!(v[0] == 1);
    assert!(v[1] == 2);
    assert!(v[2] == 3);
    assert!(v[3] == 4);
}

#[inline(always)]
pub fn clone<T: Num>(v: [T; 4]) -> [T; 4] {create(v[0], v[1], v[2], v[3])}

#[inline(always)]
pub fn copy<T: Num>(out: &mut [T; 4], a: [T; 4]) -> &mut [T; 4] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out
}
#[test]
fn test_copy() {
    let mut v = [0, 0, 0, 0];
    copy(&mut v, [1, 2, 3, 4]);
    assert!(v == [1, 2, 3, 4]);
}
