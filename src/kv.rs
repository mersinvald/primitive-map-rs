use hash::Hash;

pub trait Key: Sized + Copy + Default + PartialEq + Eq + Hash + Ord {}
impl<T> Key for T
where
    T: Sized + Copy + Default + PartialEq + Eq + Hash + Ord,
{
}

pub trait Value: Sized + Clone + Default {}
impl<T> Value for T
where
    T: Sized + Clone + Default,
{
}
