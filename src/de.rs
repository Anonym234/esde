use std::fmt::Display;

/// an error that can occur during deserialization
#[derive(Debug)]
pub enum Error<SenderError: std::error::Error> {
    /// end of file when another `Item` was expected
    EOF,
    /// error within the [`Sender`], e.g. a [`std::io::Error`] if a [`std::fs::File`] was used and an error occured while reading
    Sender(SenderError),
    /// an error while parsing, hinting corrupt data or parsing of another type was wasn't stored
    Parse(Box<dyn std::error::Error>),
}

impl<T: std::error::Error> From<&str> for Error<T> {
    fn from(value: &str) -> Self {
        Self::Parse(value.into())
    }
}

impl<T: std::error::Error> From<String> for Error<T> {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl<SenderError: std::error::Error> Display for Error<SenderError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EOF => write!(f, "reached EOF before expecting it !"),
            Error::Sender(err) => write!(
                f,
                "the sender (the instance providing items) had an error: {err:?}"
            ),
            Error::Parse(err) => write!(f, "an error occured while parsing: {err:?}"),
        }
    }
}

impl<T: std::error::Error> std::error::Error for Error<T> {}

impl<SenderError: std::error::Error> Error<SenderError> {
    /// make a given error a parse error
    ///
    /// useful when parsing to do something like
    /// ```rust
    /// let buffer = receiver.auto()?;
    /// let str = String::from_utf8(buffer).map_err(Error::make_parse)?;
    /// ```
    pub fn make_parse(err: impl std::error::Error + 'static) -> Self {
        Self::Parse(Box::new(err))
    }

    /// unwrap to a sender error only
    ///
    /// useful when in main function that, for example, returns [`std::io::Error`] to only see sender errors
    ///
    /// # panics
    /// this function panics if the contained error is *not* a sender error
    pub fn unwrap_sender(self) -> SenderError {
        match self {
            Error::EOF => panic!("unwrapped a EOF error"),
            Error::Sender(sender) => sender,
            Error::Parse(parse) => panic!("unwrapped a parse error: {parse}"),
        }
    }
}

/// trait for an object than can send/supply some kind of `Item`
pub trait Sender {
    /// the type of item supplied, e.g. [`u8`]s
    type Item;
    /// the type of error that can occur when an `Item` is requested
    type Error: std::error::Error;

    /// get the next item
    fn get(&mut self) -> Result<Self::Item, Error<Self::Error>>;

    /// fill a buffer of items (completely)
    ///
    /// # Note
    /// The canonical implementation calls [`Self::get`] as often as necessary, throwing the corresponding errors.
    /// This function should be overloaded if a better implementation can be made.
    fn fill_buffer(&mut self, buffer: &mut [Self::Item]) -> Result<(), Error<Self::Error>> {
        for i in 0..buffer.len() {
            buffer[i] = self.get()?;
        }
        Ok(())
    }

    /// get a buffer of `N` items that is completely filled
    ///
    /// # Note
    /// The canonical implementation calls [`Self::get`] as often as necessary, throwing the corresponding errors.
    /// This function should be overloaded if a better implementation can be made.
    fn get_buffer<const N: usize>(&mut self) -> Result<[Self::Item; N], Error<Self::Error>> {
        let mut buffer = Vec::new();
        for _ in 0..N {
            buffer.push(self.get()?);
        }
        let Ok(buffer) = buffer.try_into() else { unreachable!() };
        Ok(buffer)
    }

    /// automatically parse the (usually inferred) output type that implements [`Deserialize`]
    fn auto<D: Deserialize<Self::Item> + ?Sized>(&mut self) -> Result<D, Error<Self::Error>> {
        D::deserialize(self)
    }

    /// alias for [`Self::auto`] to be used in cases when a type implements [`Sender`] *and* [`crate::Receiver`]
    fn auto_de<D: Deserialize<Self::Item> + ?Sized>(&mut self) -> Result<D, Error<Self::Error>> {
        self.auto()
    }
}

/// trait for an object that can be deserialized from a stream of `Item`s
pub trait Deserialize<Item>: Sized {
    /// given a sender of type `S`, deserialize [`Self`] from `sender`'s items or throw an error
    fn deserialize<S: Sender<Item = Item> + ?Sized>(
        sender: &mut S,
    ) -> Result<Self, Error<S::Error>>;
}
