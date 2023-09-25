/// trait for an object that can accept some kind of `Item`
pub trait Receiver: Sized {
    /// the type of item that can be accepted, e.g. [`u8`]
    type Item;
    /// the type of error that can occur when a [`Self::Item`] cannot be accepted
    type Error;

    /// accept an item, return error if it cannot be accepted
    fn accept(&mut self, item: Self::Item) -> Result<(), Self::Error>;

    /// accept a buffer of cloneable items
    ///
    /// # Note
    /// The canonical implementation calls [`Self::accept`] as often as necessary.
    /// This method should be overidden if a better implementation can be made.
    fn accept_buffer(&mut self, items: &[Self::Item]) -> Result<(), Self::Error>
    where
        Self::Item: Clone,
    {
        for item in items {
            self.accept(item.clone())?;
        }
        Ok(())
    }

    /// automatically serialize a given object
    fn auto(&mut self, obj: impl Serialize<Self::Item>) -> Result<(), Self::Error> {
        obj.serialize(self)
    }

    /// alias for [`Self::auto`] in case a type implements [`crate::Sender`] *and* [`Receiver`]
    fn auto_ser(&mut self, obj: impl Serialize<Self::Item>) -> Result<(), Self::Error> {
        self.auto(obj)
    }
}

/// trait for an object that can be serialized into `Item`s
pub trait Serialize<Item> {
    /// serialize `Self` into the given receiver
    fn serialize<R: Receiver<Item = Item>>(self, receiver: &mut R) -> Result<(), R::Error>;
}
