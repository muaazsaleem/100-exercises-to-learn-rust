use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
struct SaturatingU16 {
    value: u16
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16{
            value
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16{
            value: *value
        }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16{
            value: value.into()
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16{
            value: (*value).into()
        }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16{
            value: self.value.saturating_add(rhs.value)
        }

    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16{
            value: self.value.saturating_add(rhs.value)
        }

    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
       SaturatingU16{
           value: self.value.saturating_add(rhs.into())
       }

    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16{
            value: self.value.saturating_add((*rhs).into())
        }

    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
       self.value == *other
    }
}


#[test]
fn test_saturating_u16() {
    let a: SaturatingU16 = (&10u8).into();
    let b: SaturatingU16 = 5u8.into();
    let c: SaturatingU16 = u16::MAX.into();
    let d: SaturatingU16 = (&1u16).into();
    let e = &c;

    assert_eq!(a + b, SaturatingU16::from(15u16));
    assert_eq!(a + c, SaturatingU16::from(u16::MAX));
    assert_eq!(a + d, SaturatingU16::from(11u16));
    assert_eq!(a + a, 20u16);
    assert_eq!(a + 5u16, 15u16);
    assert_eq!(a + e, SaturatingU16::from(u16::MAX));
}
