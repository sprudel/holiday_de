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