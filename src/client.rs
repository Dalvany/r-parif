//! Client that allow to make request to AirParif services and
//! convert JSON result into objects
extern crate reqwest;

use chrono::{Date, Duration, NaiveDate, Utc};
use json::JsonValue;
use reqwest::blocking::Client;
use reqwest::blocking::Response;

use crate::error::RParifError;
use crate::objects::{Criteria, Day, Episode, Index};

/// Client to call HTTP API
pub struct RParifClient<'a> {
    /// HTTP client
    client: Client,
    /// API key
    api_key: &'a str,
    /// Base URL
    base_url: &'a str,
}

impl RParifClient<'_> {
    /// Construct a new client
    ///
    /// # Arguments
    ///
    /// * `api_key` - [API key](https://www.airparif.asso.fr/rss/api) to authenticate call
    ///
    pub fn new(api_key: &str) -> RParifClient {
        RParifClient {
            client: Client::new(),
            api_key,
            base_url: "https://www.airparif.asso.fr/services/api/1.1",
        }
    }

    /// Constructor used for test with httpmock. It use `http://localhost:5000`
    /// as URL to call mock instead of real services
    ///
    /// # Arguments
    ///
    /// * `api_key` - any string
    ///
    pub fn new_test(api_key: &str) -> RParifClient {
        RParifClient {
            client: Client::new(),
            api_key,
            base_url: "http://localhost:5000",
        }
    }

    /// Convert a value into a date
    ///
    /// # Arguments
    ///
    /// * `value` - `JsonValue::String` containing one of the following `hier`, `jour` or `demain`
    ///
    /// # Errors
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the value doesn't match `hier`, `jour` or `demain`
    ///
    fn convert_json_to_date(&self, value: &JsonValue) -> Result<Date<Utc>, RParifError> {
        let date = chrono::Utc::today();

        if value == "hier" {
            Ok(date.checked_sub_signed(Duration::days(1)).unwrap())
        } else if value == "demain" {
            Ok(date.checked_add_signed(Duration::days(1)).unwrap())
        } else if value == "jour" {
            Ok(date)
        } else {
            Err(RParifError::UnexpectedDate(value.dump()))
        }
    }

    /// Convert a value into a date
    ///
    /// # Arguments
    ///
    /// * `value` - string containing one of the following `hier`, `jour` or `demain`
    ///
    /// # Errors
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the value doesn't match `hier`, `jour` or `demain`
    ///
    fn convert_string_to_date(&self, value: &str) -> Result<Date<Utc>, RParifError> {
        let date = chrono::Utc::today();

        if value == "hier" {
            Ok(date.checked_sub_signed(Duration::days(1)).unwrap())
        } else if value == "demain" {
            Ok(date.checked_add_signed(Duration::days(1)).unwrap())
        } else if value == "jour" {
            Ok(date)
        } else {
            Err(RParifError::UnexpectedDate(value.to_string()))
        }
    }

    /// Extract a number value from a JsonValue object
    ///
    /// # Arguments
    ///
    /// * `key` - Member name of the JSON value
    ///
    /// * `json` - JsonValue::Object
    ///
    /// # Errors
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) when `json`
    /// contains no member `key`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) when member
    /// `key`is not a number
    fn get_number_value(&self, key: &str, json: &JsonValue) -> Result<u32, RParifError> {
        if !json.has_key(key) {
            Err(RParifError::MissingJsonKey {
                key: key.to_string(),
                json: json.dump(),
            })
        } else if !json[key].is_number() {
            Err(RParifError::WrongJsonType {
                expected: "number".to_string(),
                json: json[key].dump(),
            })
        } else {
            Ok(json[key].as_u32().unwrap())
        }
    }

    /// Extract a string value from a JsonValue object
    ///
    /// # Arguments
    ///
    /// * `key` - Member name of the JSON value
    ///
    /// * `json` - JsonValue::Object
    ///
    /// # Errors
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) when `json`
    /// contains no member `key`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) when member
    /// `key`is not a string
    fn get_string_value<'a>(&self, key: &str, json: &'a JsonValue) -> Result<&'a str, RParifError> {
        if !json.has_key(key) {
            Err(RParifError::MissingJsonKey {
                key: key.to_string(),
                json: json.dump(),
            })
        } else if !json[key].is_string() {
            Err(RParifError::WrongJsonType {
                expected: "string".to_string(),
                json: json[key].dump(),
            })
        } else {
            Ok(json[key].as_str().unwrap())
        }
    }

    /// Execute a query to HTTP AirParif endpoint and return the body
    /// content as a string.
    ///
    /// # Arguments
    ///
    /// * `url` - URL to call
    ///
    /// # Errors
    ///
    /// * [RParifError::RequestError](../error/enum.RParifError.html#variant.RequestError) when reqwest lib
    /// fails. It contains the underlying error.
    ///
    /// * [RParifError::CallError](../error/enum.RParifError.html#variant.CallError) when HTTP status is
    /// other than 2XX. It contains the URL called, the HTTP status and the body response
    ///
    /// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
    /// formed JSON
    fn execute_query(&self, url: &str) -> Result<JsonValue, RParifError> {
        let response: Response = self.client.get(url).send()?;
        let status: u16 = response.status().as_u16();
        let ok: bool = response.status().is_success();
        let data: JsonValue = json::parse(response.text()?.as_str())?;

        if ok {
            Ok(data)
        } else {
            Err(RParifError::CallError {
                url: url.to_string(),
                body: data.dump(),
                status,
            })
        }
    }

    /// This method converts indice's JSON response into a list of
    /// [`Index`](../objects/struct.Index.html)
    ///
    /// # Arguments
    ///
    /// * `json` - HTTP body as JsonValue
    ///
    /// # Errors
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the date can't be parsed (see [`convert_json_to_date`](#method.convert_json_to_date))
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `indice`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `indice`
    /// is not a number or if JSON is not as expected
    fn index_to_index(&self, json: JsonValue) -> Result<Vec<Index>, RParifError> {
        debug!("Indice json : {}", json);
        let mut result: Vec<Index> = Vec::new();
        match &json {
            JsonValue::Array(data) => {
                for value in data {
                    debug!("Converting : {}", value);

                    // Getting date, raising error
                    let date = self.convert_json_to_date(&value["date"])?.naive_utc();

                    // Getting url
                    let url = if value.has_key("url_carte") {
                        Option::from(value["url_carte"].to_string())
                    } else {
                        None
                    };

                    let index = self.get_number_value("indice", &value)?;

                    result.push(Index::new(
                        date,
                        url,
                        vec!["global".to_string()],
                        index,
                        None,
                    ))
                }
                debug!("Result : {:?}", result);
                Ok(result)
            }
            _ => Err(RParifError::WrongJsonType {
                expected: "array".to_string(),
                json: json.dump(),
            }),
        }
    }

    /// This method converts indiceJour's JSON response into a list of  [`Index`](../objects/struct.Index.html)
    ///
    /// # Arguments
    ///
    /// * `json` - HTTP body as JsonValue
    ///
    /// # Errors
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `indice` or `date`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `indice`
    /// is not a number or if `date` isn't a string
    ///
    /// * [RParifError::DateParseError](../error/enum.RParifError.html#variant.DateParseError) if `date`
    /// is not in `dd/mm/yyyy` format
    fn index_day_to_index(&self, json: JsonValue) -> Result<Vec<Index>, RParifError> {
        debug!("Indice day json : {}", json);
        let mut result: Vec<Index> = Vec::new();

        // Getting date from json
        let date = self.get_string_value("date", &json)?;
        let date: NaiveDate = NaiveDate::parse_from_str(date, "%d/%m/%Y")
            .or_else(|error| Err(RParifError::DateParseError(error)))?;
        debug!("Date : {}", date);

        for (key, value) in json.entries() {
            if key != "date" {
                debug!("Converting : {}", value);
                let index = self.get_number_value("indice", value)?;
                let url = self
                    .get_string_value("url_carte", value)
                    .ok()
                    .map(|v| v.to_string());
                result.push(Index::new(date, url, vec![key.to_string()], index, None));
            }
        }

        debug!("Result : {:?}", result);
        return Ok(result);
    }

    /// This method converts idxville's JSON response into a list of  [`Index`](../objects/struct.Index.html)
    ///
    /// # Arguments
    ///
    /// * `json` - HTTP body as JsonValue
    ///
    /// # Errors
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the date can't be parsed (see [`convert_string_to_date`](#method.convert_string_to_date))
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `ninsee` or `indice`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `ninsee`
    /// isn't a string or `indice` is not a number or if JSON is not as expected
    fn idxville_to_index(&self, json: JsonValue) -> Result<Vec<Index>, RParifError> {
        debug!("Idxville json : {}", json);
        let mut result: Vec<Index> = Vec::new();

        match json {
            JsonValue::Array(data) => {
                for i in data {
                    debug!("Converting : {}", i);
                    let insee: &str = self.get_string_value("ninsee", &i)?;
                    debug!("City code : {}", insee);
                    for (key, value) in i.entries() {
                        if key != "ninsee" {
                            debug!("Key : {}", key);
                            debug!("Converting : {}", value);
                            let date: NaiveDate = self.convert_string_to_date(key)?.naive_utc();
                            let index = self.get_number_value("indice", value)?;
                            let pollutants: Vec<String> = match &value["polluants"] {
                                JsonValue::Array(p) => p
                                    .into_iter()
                                    .map(|v| v.as_str().unwrap().to_string())
                                    .collect(),
                                _ => Vec::new(),
                            };
                            result.push(Index::new(
                                date,
                                None,
                                pollutants,
                                index,
                                Some(insee.to_string()),
                            ));
                        }
                    }
                }

                debug!("Result : {:?}", result);
                Ok(result)
            }
            _ => Err(RParifError::WrongJsonType {
                expected: "array".to_string(),
                json: json.dump(),
            }),
        }
    }

    /// This method converts episode's JSON response into a list of  [`Episode`](../objects/struct.Episode.html)
    ///
    /// # Arguments
    ///
    /// * `json` - HTTP body as JsonValue
    ///
    /// # Errors
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the date can't be parsed (see [`convert_json_to_date`](#method.convert_json_to_date))
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `type` or `niveau`
    ///
    /// * [RParifError::UnkownEnumValue](../error/enum.RParifError.html#variant.UnkownEnumValue) if  `type`
    /// or `niveau` can't be converted into corresponding enum variant
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `type`
    ///  or `niveau` is not a string or if JSON is not as expected
    fn episode_to_episode(&self, json: JsonValue) -> Result<Vec<Episode>, RParifError> {
        let mut result: Vec<Episode> = Vec::new();

        match &json {
            JsonValue::Array(data) => {
                for j in data {
                    let date = self.convert_json_to_date(&j["date"])?.naive_utc();
                    let detail: Option<String> = j["detail"].as_str().and_then(|v| {
                        if v.is_empty() {
                            None
                        } else {
                            Some(v.to_string())
                        }
                    });
                    let mut episode = Episode::new(date, detail);
                    for (key, value) in j.entries() {
                        if key != "date" && key != "detail" {
                            let pollutant = key.to_string();
                            let kind = self.get_string_value("type", value)?.parse()?;
                            let level = self.get_string_value("niveau", value)?.parse()?;
                            let criteria: Vec<Criteria> = match &value["criteres"] {
                                JsonValue::Array(v) => v
                                    .into_iter()
                                    .map(|v| v.as_str().unwrap().parse().unwrap()) // TODO handle errors ?
                                    .collect(),
                                _ => Vec::new(),
                            };
                            episode.add(pollutant, kind, level, criteria);
                        }
                    }
                    result.push(episode);
                }

                debug!("Result : {:?}", result);
                Ok(result)
            }
            _ => Err(RParifError::WrongJsonType {
                expected: "array".to_string(),
                json: json.dump(),
            }),
        }
    }

    /// Access global pollution index for previous day, current day and next day through `indice` endpoint
    ///
    /// # Errors
    ///
    /// * [RParifError::RequestError](../error/enum.RParifError.html#variant.RequestError) when reqwest lib
    /// fails. It contains the underlying error.
    ///
    /// * [RParifError::CallError](../error/enum.RParifError.html#variant.CallError) when HTTP status is
    /// other than 2XX. It contains the URL called, the HTTP status and the body response
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the date can't be parsed (see [`convert_json_to_date`](#method.convert_json_to_date))
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `ìndice`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `ìndice`
    /// is not a number or if JSON is not as expected
    ///
    /// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
    /// formed JSON
    pub fn index(&self) -> Result<Vec<Index>, RParifError> {
        debug!("Querying indice endpoint");
        // api key is not really needed here...
        let response: JsonValue =
            self.execute_query(format!("{}/indice?key={}", self.base_url, self.api_key).as_str())?;
        self.index_to_index(response)
    }

    /// Retrieve index pollution (global and per pollutant) for a given date (previous day, current or next day) using
    /// `indiceJour` endpoint
    ///
    /// # Arguments
    ///
    /// * `day` - Which day to get indices pollution for
    ///
    /// # Errors
    ///
    /// * [RParifError::RequestError](../error/enum.RParifError.html#variant.RequestError) when reqwest lib
    /// fails. It contains the underlying error.
    ///
    /// * [RParifError::CallError](../error/enum.RParifError.html#variant.CallError) when HTTP status is
    /// other than 2XX. It contains the URL called, the HTTP status and the body response
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `ìndice` or `date`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `ìndice`
    /// is not a number or if `date` isn't a string
    ///
    /// * [RParifError::DateParseError](../error/enum.RParifError.html#variant.DateParseError) if `date`
    /// is not in `dd/mm/yyyy` format
    ///
    /// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
    /// formed JSON
    pub fn index_day(&self, day: Day) -> Result<Vec<Index>, RParifError> {
        debug!("Querying indiceJour endpoint");
        // api key is not really needed here...
        let tmp = match day {
            Day::Yesterday => "hier",
            Day::Today => "jour",
            Day::Tomorrow => "demain",
        };
        let response: JsonValue = self.execute_query(
            format!(
                "{}/indiceJour?date={}&key={}",
                self.base_url, tmp, self.api_key
            )
            .as_str(),
        )?;
        self.index_day_to_index(response)
    }

    /// Allow to get pollution indices for multiple cities through `idxville` endpoint.  
    /// The indes is the combination of indices for all [pollutants](../objects/struct.Index.html#method.pollutants) listed in [index](../objects/struct.Index.html)
    ///
    /// # Arguments
    ///
    /// * `cities` - List of INSEE city code. See [here](https://data.opendatasoft.com/explore/dataset/correspondance-code-insee-code-postal%40public/table/)
    /// or [here](https://www.data.gouv.fr/en/datasets/correspondance-entre-les-codes-postaux-et-codes-insee-des-communes-francaises/) to find corresponding code
    ///
    /// # Errors
    ///
    /// * [RParifError::RequestError](../error/enum.RParifError.html#variant.RequestError) when reqwest lib
    /// fails. It contains the underlying error.
    ///
    /// * [RParifError::CallError](../error/enum.RParifError.html#variant.CallError) when HTTP status is
    /// other than 2XX. It contains the URL called, the HTTP status and the body response
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the date can't be parsed (see [`convert_string_to_date`](#method.convert_string_to_date))
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `ninsee` or `indice`
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `ninsee`
    /// isn't a string or `indice` is not a number or if JSON is not as expected
    ///
    /// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
    /// formed JSON
    pub fn index_city(&self, cities: Vec<&str>) -> Result<Vec<Index>, RParifError> {
        debug!("Querying idxville endpoint");
        let cities = cities.join(",");
        let response: JsonValue = self.execute_query(
            format!(
                "{}/idxville?villes={}&key={}",
                self.base_url, cities, self.api_key
            )
            .as_str(),
        )?;
        self.idxville_to_index(response)
    }

    /// List pollution alert for previous day, current day and next day using `episode` endpoint
    ///
    /// # Errors
    ///
    /// * [RParifError::RequestError](../error/enum.RParifError.html#variant.RequestError) when reqwest lib
    /// fails. It contains the underlying error.
    ///
    /// * [RParifError::CallError](../error/enum.RParifError.html#variant.CallError) when HTTP status is
    /// other than 2XX. It contains the URL called, the HTTP status and the body response
    ///
    /// * [RParifError::UnexpectedDate](../error/enum.RParifError.html#variant.UnexpectedDate)
    /// if the date can't be parsed (see [`convert_json_to_date`](#method.convert_json_to_date))
    ///
    /// * [RParifError::MissingJsonKey](../error/enum.RParifError.html#variant.MissingJsonKey) if missing
    /// JSON is missing `type` or `niveau`
    ///
    /// * [RParifError::UnkownEnumValue](../error/enum.RParifError.html#variant.UnkownEnumValue) if  `type`
    /// or `niveau` can't be converted into corresponding enum variant
    ///
    /// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `type`
    ///  or `niveau` is not a string or if JSON is not as expected
    ///
    /// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
    /// formed JSON
    pub fn episode(&self) -> Result<Vec<Episode>, RParifError> {
        debug!("Querying episode endpoint");
        let response: JsonValue =
            self.execute_query(format!("{}/episode?key={}", self.base_url, self.api_key).as_str())?;
        self.episode_to_episode(response)
    }
}

#[cfg(test)]
mod test {}
