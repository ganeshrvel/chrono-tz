#![allow(non_camel_case_types, clippy::unreadable_literal)]


use core::fmt::{self, Debug, Display, Formatter};
use core::str::FromStr;

use ::timezone_impl::{TimeSpans, FixedTimespanSet, FixedTimespan};

/// TimeZones built at compile time from the tz database
///
/// This implements [`chrono::TimeZone`] so that it may be used in and to
/// construct chrono's DateTime type. See the root module documentation
/// for details.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tz {
    /// Asia/Calcutta
    Asia__Calcutta,
    /// Asia/Kolkata
    Asia__Kolkata,
    /// Etc/UCT
    Etc__UCT,
    /// Etc/UTC
    Etc__UTC,
    /// Etc/Universal
    Etc__Universal,
    /// Etc/Zulu
    Etc__Zulu,
    /// UCT
    UCT,
    /// UTC
    UTC,
    /// Universal
    Universal,
    /// Zulu
    Zulu,
}
static TIMEZONES: ::phf::Map<&'static str, Tz> =
    ::phf::Map {
        key: 14108922650502679131,
        disps: &[
            (3, 0),
            (9, 0),
        ],
        entries: &[
            ("Asia/Calcutta", Tz::Asia__Calcutta),
            ("Asia/Kolkata", Tz::Asia__Kolkata),
            ("Etc/UTC", Tz::Etc__UTC),
            ("UCT", Tz::UCT),
            ("UTC", Tz::UTC),
            ("Etc/UCT", Tz::Etc__UCT),
            ("Etc/Universal", Tz::Etc__Universal),
            ("Zulu", Tz::Zulu),
            ("Etc/Zulu", Tz::Etc__Zulu),
            ("Universal", Tz::Universal),
        ],
    };
#[cfg(feature = "std")]
pub type ParseError = String;
#[cfg(not(feature = "std"))]
pub type ParseError = &'static str;

impl FromStr for Tz {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "std")]
            return TIMEZONES.get(s).cloned().ok_or_else(|| format!("'{}' is not a valid timezone", s));
        #[cfg(not(feature = "std"))]
            return TIMEZONES.get(s).cloned().ok_or("received invalid timezone");
    }
}

impl Tz {
    pub fn name(self) -> &'static str {
        match self {
            Tz::Asia__Calcutta => "Asia/Calcutta",
            Tz::Asia__Kolkata => "Asia/Kolkata",
            Tz::Etc__UCT => "Etc/UCT",
            Tz::Etc__UTC => "Etc/UTC",
            Tz::Etc__Universal => "Etc/Universal",
            Tz::Etc__Zulu => "Etc/Zulu",
            Tz::UCT => "UCT",
            Tz::UTC => "UTC",
            Tz::Universal => "Universal",
            Tz::Zulu => "Zulu",
        }
    }
}
impl Debug for Tz {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name().as_ref())
    }
}

impl Display for Tz {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name().as_ref())
    }
}

impl TimeSpans for Tz {
    fn timespans(&self) -> FixedTimespanSet {
        match *self {
            Tz::Asia__Calcutta => {
                const REST: &[(i64, FixedTimespan)] = &[
                    (-3645237208, FixedTimespan { utc_offset: 21200, dst_offset: 0, name: "HMT" }),
                    (-3155694800, FixedTimespan { utc_offset: 19270, dst_offset: 0, name: "MMT" }),
                    (-2019705670, FixedTimespan { utc_offset: 19800, dst_offset: 0, name: "IST" }),
                    (-891581400, FixedTimespan { utc_offset: 19800, dst_offset: 3600, name: "+0630" }),
                    (-872058600, FixedTimespan { utc_offset: 19800, dst_offset: 0, name: "IST" }),
                    (-862637400, FixedTimespan { utc_offset: 19800, dst_offset: 3600, name: "+0630" }),
                    (-764145000, FixedTimespan { utc_offset: 19800, dst_offset: 0, name: "IST" }),
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 21208,
                        dst_offset: 0,
                        name: "LMT",
                    },
                    rest: REST
                }
            },

            Tz::Asia__Kolkata => {
                const REST: &[(i64, FixedTimespan)] = &[
                    (-3645237208, FixedTimespan { utc_offset: 21200, dst_offset: 0, name: "HMT" }),
                    (-3155694800, FixedTimespan { utc_offset: 19270, dst_offset: 0, name: "MMT" }),
                    (-2019705670, FixedTimespan { utc_offset: 19800, dst_offset: 0, name: "IST" }),
                    (-891581400, FixedTimespan { utc_offset: 19800, dst_offset: 3600, name: "+0630" }),
                    (-872058600, FixedTimespan { utc_offset: 19800, dst_offset: 0, name: "IST" }),
                    (-862637400, FixedTimespan { utc_offset: 19800, dst_offset: 3600, name: "+0630" }),
                    (-764145000, FixedTimespan { utc_offset: 19800, dst_offset: 0, name: "IST" }),
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 21208,
                        dst_offset: 0,
                        name: "LMT",
                    },
                    rest: REST
                }
            },

            Tz::Etc__UCT => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::Etc__UTC => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::Etc__Universal => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::Etc__Zulu => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::UCT => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::UTC => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::Universal => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

            Tz::Zulu => {
                const REST: &[(i64, FixedTimespan)] = &[
                ];
                FixedTimespanSet {
                    first: FixedTimespan {
                        utc_offset: 0,
                        dst_offset: 0,
                        name: "UTC",
                    },
                    rest: REST
                }
            },

        }
    }
}
/// An array of every known variant
///
/// Useful for iterating over known timezones:
///
/// ```
/// use chrono_tz::{TZ_VARIANTS, Tz};
/// assert!(TZ_VARIANTS.iter().any(|v| *v == Tz::UTC));
/// ```
pub static TZ_VARIANTS: [Tz; 10] = [
    Tz::Asia__Calcutta,
    Tz::Asia__Kolkata,
    Tz::Etc__UCT,
    Tz::Etc__UTC,
    Tz::Etc__Universal,
    Tz::Etc__Zulu,
    Tz::UCT,
    Tz::UTC,
    Tz::Universal,
    Tz::Zulu,
];
