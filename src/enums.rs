#[derive(Clone, Copy)]
pub struct Starter(usize);

impl Starter {
    pub fn new() -> Self {
        Self(0usize)
    }
}

impl Default for Starter {
    fn default() -> Self {
        Starter::new()
    }
}

pub trait Enums {
    type Output;
    fn enums(&self) -> Vec<(usize, Self::Output)>
    where
        Self::Output: Clone;
    fn enums_start_at(&self, at: Starter) -> Vec<(usize, Self::Output)>
    where
        Self::Output: Clone;
    fn enums_mut(&mut self) -> Vec<(usize, &mut Self::Output)>;
}

impl<T: Clone> Enums for Vec<T> {
    type Output = T;
    fn enums(&self) -> Vec<(usize, Self::Output)> {
        self.iter()
            .enumerate()
            .map(|(i, v)| (i, v.clone()))
            .collect()
    }

    fn enums_start_at(&self, at: Starter) -> Vec<(usize, Self::Output)> {
        self.iter()
            .enumerate()
            .map(|(i, v)| (i + at.0, v.clone()))
            .collect()
    }

    fn enums_mut(&mut self) -> Vec<(usize, &mut Self::Output)> {
        self.iter_mut().enumerate().collect()
    }
}

impl<T: Clone> Enums for [T] {
    type Output = T;
    fn enums(&self) -> Vec<(usize, Self::Output)> {
        self.iter()
            .enumerate()
            .map(|(i, v)| (i, v.clone()))
            .collect()
    }

    fn enums_start_at(&self, at: Starter) -> Vec<(usize, Self::Output)> {
        self.iter()
            .enumerate()
            .map(|(i, v)| (i + at.0, v.clone()))
            .collect()
    }

    fn enums_mut(&mut self) -> Vec<(usize, &mut Self::Output)> {
        self.iter_mut().enumerate().collect()
    }
}

impl From<Starter> for usize {
    fn from(start: Starter) -> usize {
        start.0
    }
}

impl From<usize> for Starter {
    fn from(start: usize) -> Starter {
        Starter(start)
    }
}

#[cfg(test)]
mod tests;
