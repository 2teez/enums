use crate::enums::{Enums, Starter};

pub trait Partition<T, R>
where
    T: Clone + std::fmt::Debug,
    R: Clone + std::fmt::Debug,
{
    fn first(&self) -> T;
    fn second(&self) -> R;
}

pub trait Nums {
    type Output;
    fn nums(&self) -> Vec<Self::Output>
    where
        Self::Output: Clone + std::fmt::Debug;
    fn nums_starting_at(&self, at: Starter) -> Vec<Self::Output>;
}

impl<T> Nums for [T]
where
    T: Clone + std::fmt::Debug,
{
    type Output = usize;
    fn nums(&self) -> Vec<Self::Output> {
        self.enums_iter().map(|data| data.first()).collect()
    }

    fn nums_starting_at(&self, at: Starter) -> Vec<Self::Output> {
        self.enums_start_at(at)
            .into_iter()
            .map(|(i, _)| i)
            .collect()
    }
}

impl<T: Clone + std::fmt::Debug, R: Clone + std::fmt::Debug> Partition<T, R> for (T, R) {
    fn first(&self) -> T {
        self.0.clone()
    }

    fn second(&self) -> R {
        self.1.clone()
    }
}

#[cfg(test)]
mod tests;
