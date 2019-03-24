use chrono::NaiveDate;
use chrono::Duration;

enum Holidays {
    Neujahr,
    HeiligeDreiKoenige,
    Frauentag,
    Karfreitag,
    Ostersonntag,
    Ostermontag,
    ErsterMai,
    ChristiHimmelfahrt,
    Pfingstsonntag,
    Pfingstmontag,
    Fronleichnam,
    AugsburgerFriedensfest,
    MariaeHimmelfahrt,
    Weltkindertag,
    TagDerDeutschenEinheit,
    Reformationstag,
    Allerheiligen,
    BussUndBettag,
    ErsterWeihnachtsfeiertag,
    ZweiterWeihnachtsfeiertag,
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd(year, month, day)
}

fn oster_sonntag(year: i32) -> NaiveDate {
    panic!("unimplemented")
}

fn bus_und_bettag(year: i32) -> NaiveDate {
    panic!("unimplemented")
}

impl Holidays {
    fn as_date(&self, year: i32) -> NaiveDate {
        match self {
            Holidays::Neujahr => date(year, 1, 1),
            Holidays::HeiligeDreiKoenige => date(year, 1, 6),
            Holidays::Frauentag => date(year, 1, 8),
            Holidays::Karfreitag => oster_sonntag(year) - Duration::days(2),
            Holidays::Ostersonntag => oster_sonntag(year),
            Holidays::Ostermontag => oster_sonntag(year) + Duration::days(1),
            Holidays::ErsterMai => date(year, 5, 1),
            Holidays::ChristiHimmelfahrt => oster_sonntag(year) + Duration::days(39),
            Holidays::Pfingstsonntag => oster_sonntag(year) + Duration::days(49),
            Holidays::Pfingstmontag => oster_sonntag(year) + Duration::days(50),
            Holidays::Fronleichnam => oster_sonntag(year) + Duration::days(60),
            Holidays::AugsburgerFriedensfest => date(year, 8,8),
            Holidays::MariaeHimmelfahrt => date(year, 8, 15),
            Holidays::Weltkindertag => date(year, 9, 20),
            Holidays::TagDerDeutschenEinheit => date(year, 10, 3),
            Holidays::Reformationstag => date(year, 10, 31),
            Holidays::Allerheiligen => date(year, 11, 1),
            Holidays::BussUndBettag => bus_und_bettag(year),
            Holidays::ErsterWeihnachtsfeiertag => date(year, 12, 25),
            Holidays::ZweiterWeihnachtsfeiertag => date(year, 12, 26),
        }
    }
}

enum Regions {
    BadenWuerttemberg,
    Bayern,
    Berlin,
    Brandenburg,
    Bremen,
    Hamburg,
    Hessen,
    MechlenburgVorpommern,
    Niedersachsen,
    NordrheinWestfalen,
    RheinlandPfalz,
    Saarland,
    Sachsen,
    SachsenAnhalt,
    SchleswigHolstein,
    Thueringen,
}

const BUNDESWEITE_FEIERTAGE: &'static [Holidays] = &[
    Holidays::Neujahr,
    Holidays::Karfreitag,
    Holidays::Ostersonntag,
    Holidays::Ostermontag,
    Holidays::ErsterMai,
    Holidays::ChristiHimmelfahrt,
    Holidays::Pfingstsonntag,
    Holidays::Pfingstmontag,
    Holidays::TagDerDeutschenEinheit,
    Holidays::ErsterWeihnachtsfeiertag,
    Holidays::ZweiterWeihnachtsfeiertag,
];

impl Regions {
    fn region_specific_holidays(&self) -> &'static [Holidays] {
        match self {
            Regions::BadenWuerttemberg =>
                &[Holidays::HeiligeDreiKoenige,
                    Holidays::Fronleichnam,
                    Holidays::Allerheiligen],
            Regions::Bayern =>
                &[Holidays::HeiligeDreiKoenige,
                    Holidays::Fronleichnam,
                    Holidays::MariaeHimmelfahrt,
                    Holidays::Allerheiligen],
            Regions::Berlin => &[Holidays::Frauentag],
            Regions::Brandenburg => &[Holidays::Reformationstag],
            Regions::Bremen => &[Holidays::Reformationstag],
            Regions::Hamburg => &[Holidays::Reformationstag],
            Regions::Hessen => &[Holidays::Fronleichnam],
            Regions::MechlenburgVorpommern => &[Holidays::Reformationstag],
            Regions::Niedersachsen => &[Holidays::Reformationstag],
            Regions::NordrheinWestfalen => &[Holidays::Fronleichnam, Holidays::Allerheiligen],
            Regions::RheinlandPfalz => &[Holidays::Fronleichnam, Holidays::Allerheiligen],
            Regions::Saarland =>
                &[Holidays::Fronleichnam, Holidays::MariaeHimmelfahrt, Holidays::Allerheiligen],
            Regions::Sachsen => &[Holidays::Reformationstag, Holidays::BussUndBettag],
            Regions::SachsenAnhalt => &[Holidays::HeiligeDreiKoenige, Holidays::Reformationstag],
            Regions::SchleswigHolstein => &[Holidays::Reformationstag],
            Regions::Thueringen => &[Holidays::Weltkindertag, Holidays::Reformationstag],
        }
    }
}