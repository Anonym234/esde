use std::io::{Read, Write};

use crate::*;

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// Sender and Receiver
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl From<std::io::Error> for Error<std::io::Error> {
    fn from(value: std::io::Error) -> Self {
        Self::Sender(value)
    }
}

/// any [`std::io::Read`] can send [`u8`]s
impl<T: Read> Sender for T {
    type Item = u8;
    type Error = std::io::Error;

    fn get(&mut self) -> Result<Self::Item, Error<Self::Error>> {
        let mut buffer = [0; 1];
        match self.read_exact(&mut buffer) {
            Ok(_) => Ok(buffer[0]),
            Err(err) => match err.kind() {
                std::io::ErrorKind::UnexpectedEof => Err(Error::EOF),
                _ => Err(err.into()),
            },
        }
    }

    fn fill_buffer(&mut self, buffer: &mut [Self::Item]) -> Result<(), Error<Self::Error>> {
        match self.read_exact(buffer) {
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => Err(Error::EOF),
            x => Ok(x?),
        }
    }

    fn get_buffer<const N: usize>(&mut self) -> Result<[Self::Item; N], Error<Self::Error>> {
        let mut buffer = [0; N];
        self.fill_buffer(&mut buffer)?;
        Ok(buffer)
    }
}

/// any [`std::io::Write`] can receive [`u8`]s
impl<T: Write> Receiver for T {
    type Item = u8;
    type Error = std::io::Error;

    fn accept(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        self.write_all(&[item])
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// vectors / slices
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item, T> Deserialize<Item> for Vec<T>
where
    T: Deserialize<Item>,
    usize: Deserialize<Item>,
{
    /// reads vector as length given [`usize`] and then the corresponding amount of `Item`s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Vec<T>, Error<S::Error>> {
        let len = sender.auto()?;
        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(sender.auto()?);
        }
        Ok(buffer)
    }
}

impl<Item, T> Serialize<Item> for &[T]
where
    T: Serialize<Item> + Clone,
    usize: Serialize<Item>,
{
    /// saves a slice/vector (if items are cloneable) as the length (number of items) as a [`usize`] and then (clones) of the `Self::Item`s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.len())?;
        for item in self {
            receiver.auto(item.clone())?;
        }
        Ok(())
    }
}

impl<Item, T> Serialize<Item> for Vec<T>
where
    T: Serialize<Item>,
    usize: Serialize<Item>,
{
    /// saves a vector (moved here) as length (number of items) as a [`usize`] and then the [`Self::Item`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.len())?;
        for item in self {
            receiver.auto(item)?;
        }
        Ok(())
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// fixed size arrays
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<const N: usize, Item, T: Deserialize<Item>> Deserialize<Item> for [T; N] {
    /// reads `N` elements and puts them in a `[T; N]` array (no length stored)
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<[T; N], Error<S::Error>> {
        let mut buffer = Vec::with_capacity(N);
        for _ in 0..N {
            buffer.push(sender.auto()?);
        }
        let Ok(buffer) = buffer.try_into() else { unreachable!() };
        Ok(buffer)
    }
}

impl<const N: usize, Item, T> Serialize<Item> for [T; N]
where
    T: Serialize<Item>,
{
    /// saves fixed-size array simply as `N` `T`s one after another (no length stored)
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        for item in self {
            receiver.auto(item)?;
        }
        Ok(())
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// Option
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item, T> Deserialize<Item> for Option<T>
where
    T: Deserialize<Item>,
    bool: Deserialize<Item>,
{
    /// an option is read just like it would've been derived
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let is_some: bool = sender.auto()?;
        if is_some {
            Ok(Some(sender.auto()?))
        } else {
            Ok(None)
        }
    }
}

impl<Item, T> Serialize<Item> for Option<T>
where
    T: Serialize<Item>,
    bool: Serialize<Item>,
{
    /// an option is saved  just like it would've been derived
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        match self {
            Some(x) => receiver.auto((true, x)),
            None => receiver.auto(false),
        }
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// Simple Tuple
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item, A, B> Deserialize<Item> for (A, B)
where
    A: Deserialize<Item>,
    B: Deserialize<Item>,
{
    /// tuples are read as an `A` and `B` one after another
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let a = sender.auto()?;
        let b = sender.auto()?;
        Ok((a, b))
    }
}

impl<Item, A, B> Serialize<Item> for (A, B)
where
    A: Serialize<Item>,
    B: Serialize<Item>,
{
    /// tuples are saved as an `A` and `B` one after another
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let (a, b) = self;
        receiver.auto(a)?;
        receiver.auto(b)?;
        Ok(())
    }
}
