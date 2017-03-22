pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut arr = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut arr).clone_from_slice(slice);
    arr
}

pub fn shuffle_create_new<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut new_vec = vec.to_vec();
    //# shuffle
    new_vec
}