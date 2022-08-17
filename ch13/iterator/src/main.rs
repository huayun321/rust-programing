fn main() {
    //迭代器与消耗
    // let v1 = vec![1, 2, 3];
    // let iter1 = v1.iter();
    // for item in iter1 {
    //     println!("item is: {}", item);
    // }

    //迭代器的适配器
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // // ============from source
    // // iter 中的数据会消耗
    // // First, we declare a type which has the `iter` method to get the `Iter`
    // // struct (`&[usize]` here):
    // let slice = &[1, 2, 3];
    //
    // // Then, we get the iterator:
    // let mut iter = slice.iter();
    // // So if we print what `as_slice` method returns here, we have "[1, 2, 3]":
    // println!("{:?}", iter.as_slice());
    //
    // // Next, we move to the second element of the slice:
    // iter.next();
    // // Now `as_slice` returns "[2, 3]":
    // println!("{:?}", iter.as_slice());

    // ============from source
    // // First, we declare a type which has `iter_mut` method to get the `IterMut`
    // // struct (`&[usize]` here):
    // let mut slice = &mut [1, 2, 3];
    //
    // // Then, we iterate over it and increment each element value:
    // for element in slice.iter_mut() {
    //     *element += 1;
    // }
    //
    // // We now have "[2, 3, 4]":
    // println!("{:?}", slice);
}
