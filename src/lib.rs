/******************************************************************************
 * Copyright ContinuousC. Licensed under the "Elastic License 2.0".           *
 ******************************************************************************/

pub trait Wrapper {
    type Wrap<T>: Wrapped<Self, T>;
}

pub trait Wrapped<W: Wrapper + ?Sized, T> {
    //fn wrap(val: T) -> Self;
    fn get_wrapped(&self) -> &T;
    fn try_get_wrapped(&self) -> Option<&T>;
    fn get_wrapped_mut(&mut self) -> &mut T;
    fn unwrap_wrapped(self) -> T;
    fn map_wrapped<F, U>(self, f: F) -> W::Wrap<U>
    where
        F: FnOnce(T) -> U;
    fn try_map_wrapped<F, U, E>(self, f: F) -> Result<W::Wrap<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>;
}

/* Identity wrapper. */

#[derive(Debug)]
pub struct Identity;

impl Wrapper for Identity {
    type Wrap<T> = T;
}

impl<T> Wrapped<Identity, T> for T {
    fn get_wrapped(&self) -> &T {
        self
    }

    fn try_get_wrapped(&self) -> Option<&T> {
        Some(self)
    }

    fn get_wrapped_mut(&mut self) -> &mut T {
        self
    }

    fn unwrap_wrapped(self) -> T {
        self
    }

    fn map_wrapped<F, U>(self, f: F) -> <Identity as Wrapper>::Wrap<U>
    where
        F: FnOnce(T) -> U,
    {
        f(self)
    }

    fn try_map_wrapped<F, U, E>(self, f: F) -> Result<<Identity as Wrapper>::Wrap<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        f(self)
    }
}
