#![feature(track_caller)]
#![feature(try_trait)]
#![feature(specialization)]

use std::panic::Location;

pub enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> std::ops::Try for MyResult<T, E>
where
    E: Track,
{
    type Ok = T;
    type Error = E;

    fn into_result(self) -> Result<T, Self::Error> {
        match self {
            Self::Ok(t) => Ok(t),
            Self::Err(e) => Err(e),
        }
    }

    fn from_ok(v: T) -> Self {
        Self::Ok(v)
    }

    #[track_caller]
    fn from_error(mut v: Self::Error) -> Self {
        v.track(Location::caller());
        Self::Err(v)
    }
}

pub trait Track {
    fn track(&mut self, location: &'static Location<'static>);
}

default impl<T> Track for T {
    fn track(&mut self, _: &'static Location<'static>) {}
}
