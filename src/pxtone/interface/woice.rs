use crate::pxtone::util::{BoxOrRef, BoxOrMut};


pub trait Woice {
    fn name(&self) -> String;
    fn set_name(&mut self, name: String) -> Result<(), ()>;
}

// you can implement `IntoIterator` for `&dyn Woices` but not for `<W: Woices> &W`
// and the impl for `&dyn Woices` isn't very useful becuase you have to manually cast it anyway

pub trait Woices {
    type W: Woice;
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = BoxOrRef<Self::W>> + 'a>;
}

pub trait WoicesMut: Woices {
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = BoxOrMut<Self::W>> + 'a>;
}
