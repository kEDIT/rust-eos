#[cfg(feature = "contract")]
use crate::assert::*;
use eosio_macros::*;

#[derive(
    Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Default,
)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub struct Time(u64);

impl Time {
    pub const MICROSECOND: u64 = 1;
    pub const MILLISECOND: u64 = Self::MICROSECOND * 1_000;
    pub const SECOND: u64 = Self::MILLISECOND * 1_000;
    pub const MINUTE: u64 = Self::SECOND * 60;
    pub const HOUR: u64 = Self::MINUTE * 60;
    pub const DAY: u64 = Self::HOUR * 24;

    #[cfg(feature = "contract")]
    pub fn now() -> Self {
        Time(unsafe { ::eosio_sys::current_time() })
    }

    #[cfg(feature = "stdweb")]
    pub fn now() -> Self {
        let microseconds = ::stdweb::web::Date::now() * 1_000.0;
        Time(microseconds as u64)
    }

    #[cfg(feature = "js-sys")]
    pub fn now() -> Self {
        let microseconds = ::js_sys::Date::now() * 1_000.0;
        Time(microseconds as u64)
    }

    #[cfg(feature = "contract")]
    pub fn publication() -> Self {
        Time(unsafe { ::eosio_sys::publication_time() })
    }

    #[cfg(feature = "contract")]
    pub fn expiration() -> Self {
        let seconds = unsafe { ::eosio_sys::expiration() };
        Self::from_seconds(seconds)
    }

    pub fn zero() -> Self {
        Time(0)
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn microseconds(self) -> u64 {
        self.0
    }

    pub fn from_microseconds(microseconds: u64) -> Self {
        Time(microseconds)
    }

    pub fn milliseconds(self) -> u64 {
        self.0 / Self::MILLISECOND
    }

    pub fn from_milliseconds(milliseconds: u64) -> Self {
        Time(milliseconds * Self::MILLISECOND)
    }

    pub fn seconds(self) -> u32 {
        (self.0 / Self::SECOND) as u32
    }

    pub fn from_seconds(seconds: u32) -> Self {
        Time(u64::from(seconds) * Self::SECOND)
    }

    pub fn minutes(self) -> u32 {
        (self.0 / Self::MINUTE) as u32
    }

    pub fn from_minutes(minutes: u32) -> Self {
        Time(u64::from(minutes) * Self::MINUTE)
    }

    pub fn hours(self) -> u32 {
        (self.0 / Self::HOUR) as u32
    }

    pub fn from_hours(hours: u32) -> Self {
        Time(u64::from(hours) * Self::HOUR)
    }

    pub fn days(self) -> u32 {
        (self.0 / Self::HOUR) as u32
    }

    pub fn from_days(days: u32) -> Self {
        Time(u64::from(days) * Self::DAY)
    }

    pub fn max(self, other: Self) -> Self {
        if self >= other {
            self
        } else {
            other
        }
    }

    pub fn min(self, other: Self) -> Self {
        if self <= other {
            self
        } else {
            other
        }
    }
}

#[cfg(feature = "contract")]
impl crate::print::Print for Time {
    fn print(&self) {
        "Time(".print();
        self.0.print();
        ")".print();
    }
}

#[cfg(feature = "serde")]
struct TimeVisitor;

#[cfg(feature = "serde")]
impl<'de> ::serde::de::Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("a microsecond timestamp as a number or string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        match value.parse::<u64>() {
            Ok(n) => Ok(Time(n)),
            Err(e) => Err(::serde::de::Error::custom(e)),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: ::serde::de::Error,
    {
        Ok(Time(value))
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::de::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(TimeVisitor)
    }
}

// TODO: TimeSpan ops similar to std::time::Duration

#[derive(Read, Write, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct TimeSpan(u64);

impl From<u64> for Time {
    fn from(i: u64) -> Self {
        Time(i)
    }
}

impl From<Time> for u64 {
    fn from(t: Time) -> Self {
        t.0
    }
}

impl From<i64> for Time {
    fn from(i: i64) -> Self {
        Time(i as u64)
    }
}

impl From<Time> for i64 {
    fn from(t: Time) -> Self {
        t.0 as i64
    }
}

impl From<Time> for u32 {
    fn from(t: Time) -> Self {
        t.seconds()
    }
}

// impl Add for Time {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Time(self.0.checked_add(other.0).assert("addition overflow"))
//     }
// }

// impl AddAssign for Time {
//     fn add_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_add(other.0).assert("addition overflow");
//     }
// }

// impl Sub for Time {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("subtraction overflow"))
//     }
// }

// impl SubAssign for Time {
//     fn sub_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_sub(other.0).assert("subtraction overflow");
//     }
// }

// impl Mul for Time {
//     type Output = Self;
//     fn mul(self, other: Self) -> Self {
//         Time(
//             self.0
//                 .checked_mul(other.0)
//                 .assert("multiplication overflow"),
//         )
//     }
// }

// impl MulAssign for Time {
//     fn mul_assign(&mut self, other: Self) {
//         self.0 = self
//             .0
//             .checked_mul(other.0)
//             .assert("multiplication overflow");
//     }
// }

// impl Div for Time {
//     type Output = Self;
//     fn div(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("division overflow"))
//     }
// }

// impl DivAssign for Time {
//     fn div_assign(&mut self, other: Self) {
//         eosio_assert(other.0 != 0, "divide by zero");
//         self.0 = self.0.checked_div(other.0).assert("division overflow");
//     }
// }

// impl Add for Time {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         Time(self.0.checked_add(other.0).assert("addition overflow"))
//     }
// }

// impl AddAssign for Time {
//     fn add_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_add(other.0).assert("addition overflow");
//     }
// }

// impl Sub for Time {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("subtraction overflow"))
//     }
// }

// impl SubAssign for Time {
//     fn sub_assign(&mut self, other: Self) {
//         self.0 = self.0.checked_sub(other.0).assert("subtraction overflow");
//     }
// }

// impl Mul for Time {
//     type Output = Self;
//     fn mul(self, other: Self) -> Self {
//         Time(
//             self.0
//                 .checked_mul(other.0)
//                 .assert("multiplication overflow"),
//         )
//     }
// }

// impl MulAssign for Time {
//     fn mul_assign(&mut self, other: Self) {
//         self.0 = self
//             .0
//             .checked_mul(other.0)
//             .assert("multiplication overflow");
//     }
// }

// impl Div for Time {
//     type Output = Self;
//     fn div(self, other: Self) -> Self {
//         Time(self.0.checked_sub(other.0).assert("division overflow"))
//     }
// }

// impl DivAssign for Time {
//     fn div_assign(&mut self, other: Self) {
//         eosio_assert(other.0 != 0, "divide by zero");
//         self.0 = self.0.checked_div(other.0).assert("division overflow");
//     }
// }
