use hash::Hash;

pub trait Key: Sized + Copy + Default + PartialEq + Eq + Hash {}
impl<T> Key for T
where
    T: Sized + Copy + Default + PartialEq + Eq + Hash,
{
}

pub trait Value: Sized + Clone + Default {}
impl<T> Value for T
where
    T: Sized + Clone + Default,
{
}
