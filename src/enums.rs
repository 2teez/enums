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

pub trait Enums<'a> {
    type Output;
    fn enums(&'a self) -> Vec<(usize, Self::Output)>
    where
        Self::Output: Copy;
    fn enums_start_at(&'a self, at: Starter) -> Vec<(usize, Self::Output)>
    where
        Self::Output: Copy;
    fn enums_mut(&'a mut self) -> Vec<(usize, Self::Output)>;
}

impl<'a, T: Copy> Enums<'a> for Vec<T> {
    type Output = T;
    fn enums(&'a self) -> Vec<(usize, Self::Output)> {
        self.iter().enumerate().map(|(i, &v)| (i, v)).collect()
    }

    fn enums_start_at(&'a self, at: Starter) -> Vec<(usize, Self::Output)> {
        self.iter()
            .enumerate()
            .map(|(i, &v)| (i + at.0, v))
            .collect()
    }

    fn enums_mut(&mut self) -> Vec<(usize, Self::Output)> {
        self.iter_mut().enumerate().map(|(i, v)| (i, *v)).collect()
    }
}

impl<'a, T: Copy> Enums<'a> for [T] {
    type Output = T;
    fn enums(&'a self) -> Vec<(usize, Self::Output)> {
        self.iter().enumerate().map(|(i, &v)| (i, v)).collect()
    }

    fn enums_start_at(&'a self, at: Starter) -> Vec<(usize, Self::Output)> {
        self.iter()
            .enumerate()
            .map(|(i, &v)| (i + at.0, v))
            .collect()
    }

    fn enums_mut(&mut self) -> Vec<(usize, Self::Output)> {
        self.iter_mut().enumerate().map(|(i, v)| (i, *v)).collect()
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
