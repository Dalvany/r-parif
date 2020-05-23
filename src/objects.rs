//! Several objects used to represent pollution index and alerts
extern crate chrono;

use std::fmt;
use std::str::FromStr;

use chrono::NaiveDate;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::RParifError;

/// This struct represent a pollution index
#[derive(Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Index {
    /// Date of mesure
    date: NaiveDate,
    /// An url (if any) to a map show the global pollution
    url: Option<String>,
    /// Pollutants (could be global, o3, no2, pm10, so2)
    pollutants: Vec<String>,
    /// Index
    index: u32,
    /// City INSEE code
    insee: Option<String>,
}

impl Index {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `date` - Date of the pollution index
    ///
    /// * `url` - URL to a map (if any)
    ///
    /// * `pollutants` - List of pollutant
    ///
    /// * `index` - Index of pollution
    ///
    /// * `insee` - INSEE code of a city
    // Is this usefull to make a builder ?
    // see https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
    pub fn new(
        date: NaiveDate,
        url: Option<String>,
        pollutants: Vec<String>,
        index: u32,
        insee: Option<String>,
    ) -> Index {
        Index {
            date,
            url,
            pollutants,
            index,
            insee,
        }
    }

    /// Return the date of pollution index
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Return a link to a pollution map
    pub fn map_url(&self) -> Option<String> {
        self.url.clone()
    }

    /// List of pollutants that are used to compute index
    pub fn pollutants(&self) -> Vec<String> {
        self.pollutants.to_vec()
    }

    /// Pollution index
    pub fn index(&self) -> u32 {
        self.index
    }

    /// INSEE city code
    pub fn insee(&self) -> Option<String> {
        self.insee.clone()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (city : {:?}) : {:?} = {} (map : {:?})",
            self.date, self.insee, self.pollutants, self.index, self.url
        )
    }
}

/// Represent a date as use in the HTTP API
pub enum Day {
    Yesterday,
    Today,
    Tomorrow,
}

/// Represent a pollution alert
#[derive(Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Episode {
    /// Alert date
    date: NaiveDate,
    /// Alert details
    detail: Option<String>,
    /// Details for pollutants
    pollutants: Vec<PollutantEpisode>,
}

impl Episode {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `date` - Date of the alert
    ///
    /// * `detail` - Description and advice
    pub fn new(date: NaiveDate, detail: Option<String>) -> Episode {
        Episode {
            date,
            detail,
            pollutants: vec![],
        }
    }

    /// Add a new pollutant for the alert
    ///
    /// # Arguments
    ///
    /// * `pollutant` - Name of the pollutant (globale, o2, no2, so3)
    ///
    /// * `kind` - if the alert is forecasted or observed
    ///
    /// * `level` - Pollution level for `pollutant`
    ///
    /// * `criteria` - List of criteria that raised pollution alert
    pub fn add(&mut self, pollutant: String, kind: Type, level: Level, criteria: Vec<Criteria>) {
        self.pollutants.push(PollutantEpisode {
            pollutant,
            kind,
            level,
            criteria,
        })
    }

    /// Return the date of pollution alert
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    /// Return the description of the pollution alert
    pub fn detail(&self) -> Option<String> {
        self.detail.clone()
    }

    /// Return the list of pollutant
    pub fn pollutants(&self) -> Vec<PollutantEpisode> {
        self.pollutants.to_vec()
    }
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} (detail : {:?})",
            self.date, self.pollutants, self.detail
        )
    }
}

impl IntoIterator for Episode {
    type Item = PollutantEpisode;
    type IntoIter = PollutantEpisodeIter;

    fn into_iter(self) -> Self::IntoIter {
        PollutantEpisodeIter {
            episode: self,
            i: 0,
        }
    }
}

/// Allow to iterate through PollutantEpisode of an Episode
pub struct PollutantEpisodeIter {
    episode: Episode,
    i: usize,
}

impl Iterator for PollutantEpisodeIter {
    type Item = PollutantEpisode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.episode.pollutants.len() {
            // TODO handle error ?
            let episode: PollutantEpisode = self.episode.pollutants().get(self.i).unwrap().clone();
            self.i += 1;
            Some(episode)
        } else {
            None
        }
    }
}

/// Details of pollution alert
#[derive(Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PollutantEpisode {
    /// Name of the pollutant o3, no2, so2, pm10
    pollutant: String,
    /// Type of the alert
    kind: Type,
    /// Alert severity for the pollutant
    level: Level,
    /// List of criteria that raise the alert
    criteria: Vec<Criteria>,
}

impl PollutantEpisode {
    /// Return the pollutant name
    pub fn pollutant_name(&self) -> String {
        self.pollutant.clone()
    }

    /// Return alert type for the pollutant
    pub fn kind(&self) -> Type {
        self.kind.clone()
    }

    /// Return alert level for the pollutant
    pub fn level(&self) -> Level {
        self.level.clone()
    }

    /// Return criteria that raise the alert
    pub fn criteria(&self) -> Vec<Criteria> {
        self.criteria.to_vec()
    }
}

impl fmt::Display for PollutantEpisode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} : {:?} {:?} {:?}",
            self.pollutant, self.kind, self.level, self.criteria
        )
    }
}

/// Level of pollution alert
#[derive(Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Level {
    Info,
    Alert,
    Normal,
}

impl FromStr for Level {
    type Err = RParifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "info" {
            Ok(Level::Info)
        } else if s == "alerte" {
            Ok(Level::Alert)
        } else if s == "normal" {
            Ok(Level::Normal)
        } else {
            Err(RParifError::UnkownEnumValue(s.to_string()))
        }
    }
}

/// Type of alert
#[derive(Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    /// alert was forecast
    Forecast,
    /// alert was observed
    Observed,
}

impl FromStr for Type {
    type Err = RParifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "prevu" {
            Ok(Type::Forecast)
        } else if s == "constate" {
            Ok(Type::Observed)
        } else {
            Err(RParifError::UnkownEnumValue(s.to_string()))
        }
    }
}

/// Criteria that can raise an alert
#[derive(Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Criteria {
    /// More than 100km²
    Area,
    /// More than 10% of population
    Population,
}

impl FromStr for Criteria {
    type Err = RParifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "km" {
            Ok(Criteria::Area)
        } else if s == "pop" {
            Ok(Criteria::Population)
        } else {
            Err(RParifError::UnkownEnumValue(s.to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::objects::{Criteria, Episode, Level, Type};

    use super::chrono::{Datelike, NaiveDate, Utc};

    #[test]
    fn test_episode_iterator() {
        let today = Utc::today();
        let mut episode = Episode::new(
            NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
            None,
        );
        episode.add(
            "o3".to_string(),
            Type::Observed,
            Level::Info,
            vec![Criteria::Area, Criteria::Population],
        );
        episode.add(
            "so2".to_string(),
            Type::Observed,
            Level::Alert,
            vec![Criteria::Population],
        );
        episode.add(
            "no2".to_string(),
            Type::Observed,
            Level::Normal,
            vec![Criteria::Area],
        );

        let mut i: usize = 0;
        for pollution_episode in episode.clone() {
            assert_eq!(
                pollution_episode,
                episode.pollutants().get(i).unwrap().clone()
            );
            i += 1;
        }
    }
}
