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
pub fn zero<T: Num>() -> [T; 4] {create(T::zero(), T::zero(), T::zero(), T::zero())}
#[inline(always)]
pub fn identity<T: Num>() -> [T; 4] {create(T::zero(), T::zero(), T::zero(), T::one())}

#[inline(always)]
pub fn copy<T: Num>(v: [T; 4]) -> [T; 4] {create(v[0], v[1], v[2], v[3])}
#[inline(always)]
pub fn clone<T: Num>(v: [T; 4]) -> [T; 4] {create(v[0], v[1], v[2], v[3])}
