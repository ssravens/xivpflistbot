use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Job {
    Gunbreaker,
    Paladin,
    Gladiator,
    DarkKnight,
    Warrior,
    Marauder,
    Scholar,
    Arcanist,
    Sage,
    Astrologian,
    WhiteMage,
    Conjurer,
    Samurai,
    Dragoon,
    Ninja,
    Monk,
    Reaper,
    Bard,
    Machinist,
    Dancer,
    BlackMage,
    BlueMage,
    Summoner,
    RedMage,
    Lancer,
    Pugilist,
    Rogue,
    Thaumaturge,
    Archer
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Role {
    Tank,
    DPS,
    Healer
}

#[derive(Debug)]
#[derive(Clone)]
pub struct PFListing {
    pub title: String,
    pub author: String,
    pub flags: String,
    pub description: String,
    pub slots: Vec<Slot>,
    pub last_updated: String,
    pub expires_in: String,
    pub min_ilvl: String,
    pub data_center: String,
    pub pf_category: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Slot {
    pub available_jobs: Vec<Job>,
    pub filled: bool,
}

#[allow(dead_code)]
impl Slot {   
    pub fn to_string(&self) -> String {
        // We don't want to disclose the secret
        format!("Slot({:#?}, {})", &self.available_jobs, &self.filled)
    }
    pub fn get_emoji_string(&self) -> String {
        if self.filled {
            match self.available_jobs.first() {
                Some(x) => x.get_emoji_string(),
                None => "".to_string()
            }
        } else {
            let contains_tank = self.available_jobs.iter().any(|x| x.get_role() == Role::Tank);
            let contains_healer = self.available_jobs.iter().any(|x| x.get_role() == Role::Healer);
            let contains_dps = self.available_jobs.iter().any(|x| x.get_role() == Role::DPS);

            if contains_tank && contains_healer && contains_dps {
                "<:tankhealerdps:1010655372014538855>".to_string()
            } else if contains_tank && contains_healer && !contains_dps {
                "<:tankhealer:1010655371217621032>".to_string()
            } else if contains_tank && !contains_healer && contains_dps {
                "<:tankdps:1010655370089349181>".to_string()
            } else if contains_tank && !contains_healer && !contains_dps {
                "<:tank:1010655368919142492>".to_string()
            } else if !contains_tank && contains_healer && contains_dps {
                "<:healerdps:1010655349608562800>".to_string()
            } else if !contains_tank && contains_healer && !contains_dps {
                "<:healer:1010655348601917651>".to_string()
            } else if !contains_tank && !contains_healer && contains_dps {
                "<:dps:1010655344097247382>".to_string()
            } else {
                "".to_string()
            }
        }
    }
}

impl Job {
    pub fn get_emoji_string(&self) -> String {
        match self {
            Job::Gunbreaker => "<:gunbreaker:1010655347033256008>".to_string(),
            Job::Paladin => "<:paladin:1010655357217013811>".to_string(),
            Job::Gladiator => "<:gladiator:1010655346144051273>".to_string(),
            Job::DarkKnight => "<:darkknight:1010655342977364008>".to_string(),
            Job::Warrior => "<:warrior:1010655374224924693>".to_string(),
            Job::Marauder => "<:marauder:1010655354025152652>".to_string(),
            Job::Scholar => "<:scholar:1010655365341380638>".to_string(),
            Job::Arcanist => "<:arcanist:1010655332973948938>".to_string(),
            Job::Sage => "<:sage:1010655362946441226>".to_string(),
            Job::Astrologian => "<:astrologian:1010655335226294364>".to_string(),
            Job::WhiteMage => "<:whitemage:1010655375856517280>".to_string(),
            Job::Conjurer => "<:conjurer:1010655339743559722>".to_string(),
            Job::Samurai => "<:samurai:1010655364250861679>".to_string(),
            Job::Dragoon => "<:dragoon:1010655345074524250>".to_string(),
            Job::Ninja => "<:ffxivninja:1010655356193615882>".to_string(),
            Job::Monk => "<:monk:1010655355128262686>".to_string(),
            Job::Reaper => "<:reaper:1010655359347728445>".to_string(),
            Job::Bard => "<:bard:1010655336438444163>".to_string(),
            Job::Machinist => "<:machinist:1010655352959815730>".to_string(),
            Job::Dancer => "<:ffxivdancer:1010655341253500930>".to_string(),
            Job::BlackMage => "<:blackmage:1010655337914830919>".to_string(),
            Job::BlueMage => "<:bluemage:1010655338715938909>".to_string(),
            Job::Summoner => "<:summoner:1010655367740542976>".to_string(),
            Job::RedMage => "<:redmage:1010655360400498748>".to_string(),
            Job::Lancer => "<:lancer:1010655351525363773>".to_string(),
            Job::Pugilist => "<:pugilist:1010655358320128011>".to_string(),
            Job::Rogue => "<:rogue:1010655361595867166>".to_string(),
            Job::Thaumaturge => "<:thaumaturge:1010655373516091426>".to_string(),
            Job::Archer  => "<:archer:1010655333988962366>".to_string()
        }
    }

    pub fn get_role(&self) -> Role {
        let tanks = vec![Job::Paladin, Job::Gunbreaker, Job::DarkKnight, Job::Warrior, Job::Marauder, Job::Gladiator];
        let healers = vec![Job::Conjurer, Job::WhiteMage, Job::Scholar, Job::Astrologian, Job::Sage];

        if tanks.contains(self) {
            Role::Tank
        } else if healers.contains(self) {
            Role::Healer
        } else {
            Role::DPS
        }
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Job::Gunbreaker => write!(f, "Gunbreaker"),
            Job::Paladin => write!(f, "Paladin"),
            Job::Gladiator => write!(f, "Gladiator"),
            Job::DarkKnight => write!(f, "DarkKnight"),
            Job::Warrior => write!(f, "Warrior"),
            Job::Marauder => write!(f, "Marauder"),
            Job::Scholar => write!(f, "Scholar"),
            Job::Arcanist => write!(f, "Arcanist"),
            Job::Sage => write!(f, "Sage"),
            Job::Astrologian => write!(f, "Astrologian"),
            Job::WhiteMage => write!(f, "WhiteMage"),
            Job::Conjurer => write!(f, "Conjurer"),
            Job::Samurai => write!(f, "Samurai"),
            Job::Dragoon => write!(f, "Dragoon"),
            Job::Ninja => write!(f, "Ninja"),
            Job::Monk => write!(f, "Monk"),
            Job::Reaper => write!(f, "Reaper"),
            Job::Bard => write!(f, "Bard"),
            Job::Machinist => write!(f, "Machinist"),
            Job::Dancer => write!(f, "Dancer"),
            Job::BlackMage => write!(f, "BlackMage"),
            Job::BlueMage => write!(f, "BlueMage"),
            Job::Summoner => write!(f, "Summoner"),
            Job::RedMage => write!(f, "RedMage"),
            Job::Lancer => write!(f, "Lancer"),
            Job::Pugilist => write!(f, "Pugilist"),
            Job::Rogue => write!(f, "Rogue"),
            Job::Thaumaturge => write!(f, "Thaumaturge"),
            Job::Archer => write!(f, "Archer"),
        }
    }
}

impl FromStr for Job {

    type Err = ();

    fn from_str(input: &str) -> Result<Job, Self::Err> {
        match input {
            "PLD"  => Ok(Job::Paladin),
            "WAR"  => Ok(Job::Warrior),
            "DRK"  => Ok(Job::DarkKnight),
            "GNB"  => Ok(Job::Gunbreaker),
            "GLD"  => Ok(Job::Gladiator),
            "MRD"  => Ok(Job::Marauder),
            "WHM"  => Ok(Job::WhiteMage),
            "SCH"  => Ok(Job::Scholar),
            "AST"  => Ok(Job::Astrologian),
            "SGE"  => Ok(Job::Sage),
            "CNJ"  => Ok(Job::Conjurer),
            "ARN"  => Ok(Job::Arcanist),
            "MNK"  => Ok(Job::Monk),
            "PGL"  => Ok(Job::Pugilist),
            "DRG"  => Ok(Job::Dragoon),
            "LNC"  => Ok(Job::Lancer),
            "NIN"  => Ok(Job::Ninja),
            "ROG"  => Ok(Job::Rogue),
            "SAM"  => Ok(Job::Samurai),
            "RPR"  => Ok(Job::Reaper),
            "BRD"  => Ok(Job::Bard),
            "ARC"  => Ok(Job::Archer),
            "MCH"  => Ok(Job::Machinist),
            "DNC"  => Ok(Job::Dancer),
            "BLM"  => Ok(Job::BlackMage),
            "SMN"  => Ok(Job::Summoner),
            "BLU"  => Ok(Job::BlueMage),
            "RDM"  => Ok(Job::RedMage),
            "RGE"  => Ok(Job::Rogue),
            "THM"  => Ok(Job::Thaumaturge),
            "ACN"  => Ok(Job::Arcanist),

            _      => Err(()),
        }
    }
}

pub fn get_color_from_duty(duty_name: &str) -> u32 {
    match duty_name {
        "The Unending Coil of Bahamut (Ultimate)" => 0xfce100,
        "The Weapon's Refrain (Ultimate)" => 0x008bfc,
        "The Epic of Alexander (Ultimate)" => 0xfcaa00,
        "Dragonsong's Reprise (Ultimate)" =>  0xf12916,
        _ =>  0xf0a057
    }
}