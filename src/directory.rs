#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use Tz;

pub const UCT : Tz = Tz::UCT;
pub const UTC : Tz = Tz::UTC;
pub const Universal : Tz = Tz::Universal;
pub const Zulu : Tz = Tz::Zulu;

pub mod Asia {
    use timezones::Tz;

    pub const Calcutta : Tz = Tz::Asia__Calcutta;
    pub const Kolkata : Tz = Tz::Asia__Kolkata;
}

pub mod Etc {
    use timezones::Tz;

    pub const UCT : Tz = Tz::Etc__UCT;
    pub const UTC : Tz = Tz::Etc__UTC;
    pub const Universal : Tz = Tz::Etc__Universal;
    pub const Zulu : Tz = Tz::Etc__Zulu;
}


