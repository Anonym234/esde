use crate::*;

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// unsigned integers
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl Deserialize<u8> for u8 {
    fn deserialize<S: Sender<Item = u8> + ?Sized>(sender: &mut S) -> Result<Self, Error<S::Error>> {
        Ok(sender.get()?)
    }
}

impl Serialize<u8> for u8 {
    fn serialize<R: Receiver<Item = u8>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.accept(self)
    }
}

impl<Item> Deserialize<Item> for u16
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for u16
where
    u8: Serialize<Item>,
{
    /// saving as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.to_be_bytes())
    }
}

impl<Item> Deserialize<Item> for u32
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for u32
where
    u8: Serialize<Item>,
{
    /// saving as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.to_be_bytes())
    }
}

impl<Item> Deserialize<Item> for u64
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for u64
where
    u8: Serialize<Item>,
{
    /// saving as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.to_be_bytes())
    }
}

impl<Item> Deserialize<Item> for u128
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for u128
where
    u8: Serialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.to_be_bytes())
    }
}

impl<Item> Deserialize<Item> for usize
where
    u64: Deserialize<Item>,
{
    /// saving as [`u64`] (even if `usize::BITS < 64`)
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        assert!(Self::BITS <= 64);
        let x: u64 = sender.auto()?;
        Ok(x as Self)
    }
}

impl<Item> Serialize<Item> for usize
where
    u64: Serialize<Item>,
{
    /// reading as [`u64`] (even if `usize::BITS < 64`)
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        assert!(Self::BITS <= 64);
        receiver.auto(self as u64)
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// signed integers
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item> Deserialize<Item> for i8
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for i8
where
    u8: Serialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let buffer = self.to_be_bytes();
        receiver.auto(buffer)
    }
}

impl<Item> Deserialize<Item> for i16
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for i16
where
    u8: Serialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let buffer = self.to_be_bytes();
        receiver.auto(buffer)
    }
}

impl<Item> Deserialize<Item> for i32
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for i32
where
    u8: Serialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let buffer = self.to_be_bytes();
        receiver.auto(buffer)
    }
}

impl<Item> Deserialize<Item> for i64
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for i64
where
    u8: Serialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let buffer = self.to_be_bytes();
        receiver.auto(buffer)
    }
}

impl<Item> Deserialize<Item> for i128
where
    u8: Deserialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Ok(Self::from_be_bytes(buffer))
    }
}

impl<Item> Serialize<Item> for i128
where
    u8: Serialize<Item>,
{
    /// reading as big-endian array of [`u8`]s
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let buffer = self.to_be_bytes();
        receiver.auto(buffer)
    }
}

impl<Item> Deserialize<Item> for isize
where
    i64: Deserialize<Item>,
{
    /// saving as [`i64`] (even if `usize::BITS < 64`)
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        assert!(Self::BITS <= i64::BITS);
        let x: i64 = sender.auto()?;
        Ok(x as Self)
    }
}

impl<Item> Serialize<Item> for isize
where
    i64: Serialize<Item>,
{
    /// reading as [`i64`] (even if `usize::BITS < 64`)
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        assert!(Self::BITS <= i64::BITS);
        receiver.auto(self as i64)
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// floating point
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item> Deserialize<Item> for f32
where
    u32: Deserialize<Item>,
{
    /// reading as [`u32`]
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        Ok(Self::from_bits(sender.auto()?))
    }
}

impl<Item> Serialize<Item> for f32
where
    u32: Serialize<Item>,
{
    /// saving as [`u32`]
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.to_bits())
    }
}

impl<Item> Deserialize<Item> for f64
where
    u64: Deserialize<Item>,
{
    /// reading as [`u64`]
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        Ok(Self::from_bits(sender.auto()?))
    }
}

impl<Item> Serialize<Item> for f64
where
    u64: Serialize<Item>,
{
    /// saving as [`u64`]
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.to_bits())
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// char
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item> Deserialize<Item> for char
where
    u32: Deserialize<Item>,
{
    /// reading as [`u32`], throwing error if not a valid unicode character
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let x = sender.auto()?;
        Self::from_u32(x).ok_or(
            format!("cannot parse char from u32 0x{x:x?} (not valid unicode character)").into(),
        )
    }
}

impl<Item> Serialize<Item> for char
where
    u32: Serialize<Item>,
{
    /// saving as [`u32`]
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self as u32)
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// bool
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item> Deserialize<Item> for bool
where
    u8: Deserialize<Item>,
{
    /// reading as [`u8`] (non-zero is treated as true, zero as false)
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let x: u8 = sender.auto()?;
        Ok(x != 0)
    }
}

impl<Item> Serialize<Item> for bool
where
    u8: Serialize<Item>,
{
    /// saving as [`u8`]
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        let x = self as u8;
        receiver.auto(x)
    }
}

// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// other basic types
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

impl<Item> Deserialize<Item> for String
where
    u8: Deserialize<Item>,
{
    /// reading as vector of [`u8`]s, ṕarsing with [`String::from_utf8`] (throws error if invalid UTF-8)
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>> {
        let buffer = sender.auto()?;
        Self::from_utf8(buffer).map_err(Error::make_parse)
    }
}

impl<Item> Serialize<Item> for String
where
    u8: Serialize<Item>,
{
    /// saving as UTF-8 bytes (slice of [`u8`]s) via the [`String::as_bytes`] method
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error> {
        receiver.auto(self.as_bytes())
    }
}
