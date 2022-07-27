/** GENERATED BY BENDEC TYPE GENERATOR */
#[allow(unused_imports)]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use super::shared::*;

// primitive built-in: u8
// primitive built-in: u16

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimalKind {
  /// This is a zebra
  Zebra = 0x1001,
  Toucan = 0x1002,
}
impl Default for AnimalKind {
  fn default() -> Self {
    Self::Zebra
  }
}
impl std::convert::TryFrom<u16> for AnimalKind {
  type Error = EnumValueError;
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    match value {
      0x1001 => Ok(Self::Zebra),
      0x1002 => Ok(Self::Toucan),
      other => Err(EnumValueError::new(other, "AnimalKind")),
    }
  }
}
pub struct AnimalKindInt;
#[allow(non_upper_case_globals, dead_code)]
impl AnimalKindInt {
  pub const Zebra: u16 = 0x1001;
  pub const Toucan: u16 = 0x1002;
}

#[repr(C, packed)]
#[derive(Default, Serialize, Deserialize, Copy, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct Zebra {
  pub kind: AnimalKind,
  pub legs: u8,
}

#[repr(C, packed)]
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Toucan {
  pub kind: AnimalKind,
  pub wingspan: u16,
}

pub union Animal {
  pub zebra: Zebra,
  pub toucan: Toucan,
}

impl Serialize for Animal {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer,
  {
    unsafe {
      match self.zebra.kind {
        AnimalKind::Zebra => self.zebra.serialize(serializer),
        AnimalKind::Toucan => self.toucan.serialize(serializer),
      }
    }
  }
}

impl Animal {
  pub fn deserialize<'de, D>(de: D, disc: AnimalKind) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    match disc {
      AnimalKind::Zebra => Zebra::deserialize(de).map(|v| Animal { zebra: v }),
      AnimalKind::Toucan => Toucan::deserialize(de).map(|v| Animal { toucan: v }),
    }
  }
}

impl Animal {
  pub fn size_of(disc: AnimalKind) -> usize {
    match disc {
      AnimalKind::Zebra => std::mem::size_of::<Zebra>(),
      AnimalKind::Toucan => std::mem::size_of::<Toucan>(),
    }
  }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, ExtraDerive)]
pub enum AnimalKind2 {
  Zebra2 = 0x0001,
  Toucan2 = 0x0002,
}
impl Default for AnimalKind2 {
  fn default() -> Self {
    Self::Zebra2
  }
}
impl std::convert::TryFrom<u8> for AnimalKind2 {
  type Error = EnumValueError;
  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0x0001 => Ok(Self::Zebra2),
      0x0002 => Ok(Self::Toucan2),
      other => Err(EnumValueError::new(other, "AnimalKind2")),
    }
  }
}

#[repr(C, packed)]
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Header {
  pub animal_kind: AnimalKind2,
}

#[repr(C, packed)]
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Zebra2 {
  pub header: Header,
  pub legs: u8,
}

#[repr(C, packed)]
#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Toucan2 {
  pub header: Header,
  pub wingspan: u16,
}

pub union Animal2 {
  pub zebra_2: Zebra2,
  pub toucan_2: Toucan2,
}

impl Serialize for Animal2 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer,
  {
    unsafe {
      match self.zebra_2.header.animal_kind {
        AnimalKind2::Zebra2 => self.zebra_2.serialize(serializer),
        AnimalKind2::Toucan2 => self.toucan_2.serialize(serializer),
      }
    }
  }
}

impl Animal2 {
  pub fn deserialize<'de, D>(de: D, disc: AnimalKind2) -> Result<Self, D::Error> 
  where
    D: Deserializer<'de>,
  {
    match disc {
      AnimalKind2::Zebra2 => Zebra2::deserialize(de).map(|v| Animal2 { zebra_2: v }),
      AnimalKind2::Toucan2 => Toucan2::deserialize(de).map(|v| Animal2 { toucan_2: v }),
    }
  }
}

impl Animal2 {
  pub fn size_of(disc: AnimalKind2) -> usize {
    match disc {
      AnimalKind2::Zebra2 => std::mem::size_of::<Zebra2>(),
      AnimalKind2::Toucan2 => std::mem::size_of::<Toucan2>(),
    }
  }
}

bitflags::bitflags! {
  #[derive(Serialize, Deserialize)]
  pub struct Bitflags: u8 {
    const A    = 0b00000001;
    const B    = 0b00000010;
    const LONG = 0b00000100;
  }
}
