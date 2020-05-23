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
        let date: NaiveDate = NaiveDate::parse_from_str(date, "%d/%m/%Y")?;
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
mod test {
    extern crate httpmock;

    use chrono::Datelike;
    use httpmock::Method::GET;
    use httpmock::{mock, with_mock_server};

    use crate::objects::{Level, Type};

    use super::reqwest::Url;
    use super::*;

    #[test]
    // Return yesterday
    fn test_convert_json_to_date_hier() {
        let client = RParifClient::new("api-key");
        let json = JsonValue::String("hier".to_string());
        let result = client.convert_json_to_date(&json);

        let expected = Utc::today();
        let expected = expected.checked_sub_signed(Duration::days(1));
        assert!(result.is_ok(), "Convert JSON 'hier' fails");
        assert_eq!(result.ok(), expected);
    }

    #[test]
    // Return today
    fn test_convert_json_to_date_jour() {
        let client = RParifClient::new("api-key");
        let json = JsonValue::String("jour".to_string());
        let result = client.convert_json_to_date(&json);

        let expected = Utc::today();
        assert!(result.is_ok(), "Convert JSON 'jour' fails");
        assert_eq!(result.ok(), Some(expected));
    }

    #[test]
    // Return tomorrow
    fn test_convert_json_to_date_demain() {
        let client = RParifClient::new("api-key");
        let json = JsonValue::String("demain".to_string());
        let result = client.convert_json_to_date(&json);

        let expected = Utc::today();
        let expected = expected.checked_add_signed(Duration::days(1));
        assert!(result.is_ok(), "Convert JSON 'demain' fails");
        assert_eq!(result.ok(), expected);
    }

    #[test]
    // Return an error because date isn't 'hier', 'jour' or 'demain'
    fn test_convert_json_to_date_wrong() {
        let client = RParifClient::new("api-key");
        let json = JsonValue::String("wrong string".to_string());
        let result = client.convert_json_to_date(&json);

        assert!(result.is_err(), "Convert JSON 'wrong string' should fails");
        match result.err().unwrap() {
            RParifError::UnexpectedDate(s) => assert_eq!(s, "\"wrong string\"".to_string()),
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    // Return yesterday day
    fn test_convert_string_to_date_hier() {
        let client = RParifClient::new("api-key");
        let result = client.convert_string_to_date("hier");

        let expected = Utc::today();
        let expected = expected.checked_sub_signed(Duration::days(1));
        assert!(result.is_ok(), "Convert string 'hier' fails");
        assert_eq!(result.ok(), expected);
    }

    #[test]
    // Return today
    fn test_convert_string_to_date_jour() {
        let client = RParifClient::new("api-key");
        let result = client.convert_string_to_date("jour");

        let expected = Utc::today();
        assert!(result.is_ok(), "Convert string 'jour' fails");
        assert_eq!(result.ok(), Some(expected));
    }

    #[test]
    // Return tomorrow
    fn test_convert_string_to_date_demain() {
        let client = RParifClient::new("api-key");
        let result = client.convert_string_to_date("demain");

        let expected = Utc::today();
        let expected = expected.checked_add_signed(Duration::days(1));
        assert!(result.is_ok(), "Convert string 'demain' fails");
        assert_eq!(result.ok(), expected);
    }

    #[test]
    // Return an error because date isn't 'hier', 'jour' or 'demain'
    fn test_convert_string_to_date_wrong() {
        let client = RParifClient::new("api-key");
        let result = client.convert_string_to_date("wrong string");

        assert!(
            result.is_err(),
            "Convert string 'wrong string' should fails"
        );
        match result.err().unwrap() {
            RParifError::UnexpectedDate(s) => assert_eq!(s, "wrong string".to_string()),
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    // Get number value from JSON return an error because
    // the JSON key doesn't exists
    fn test_get_number_value_no_key() {
        let client = RParifClient::new("api-key");
        let data = object! {
            wrong_key: 12
        };

        let result = client.get_number_value("key", &data);

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::MissingJsonKey { key, json } => {
                assert_eq!(key, "key".to_string());
                assert_eq!(json, "{\"wrong_key\":12}")
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    // Get number value from JSON return an error because
    // the value is not a string
    fn test_get_number_value_wrong_type() {
        let client = RParifClient::new("api-key");
        let data = object! {
            key: "wrong type"
        };

        let result = client.get_number_value("key", &data);

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::WrongJsonType { expected, json } => {
                assert_eq!(expected, "number".to_string());
                assert_eq!(json, "\"wrong type\"")
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    // Get number value from JSON ok
    fn test_get_number_value() {
        let client = RParifClient::new("api-key");
        let data = object! {
            key: 12
        };

        let result = client.get_number_value("key", &data);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(12));
    }

    #[test]
    // Get string value from JSON return an error because
    // the JSON key doesn't exists
    fn test_get_string_value_no_key() {
        let client = RParifClient::new("api-key");
        let data = object! {
            wrong_key: "data"
        };

        let result = client.get_string_value("key", &data);

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::MissingJsonKey { key, json } => {
                assert_eq!(key, "key".to_string());
                assert_eq!(json, "{\"wrong_key\":\"data\"}")
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    // Get string value from JSON return an error because
    // the value is not a string
    fn test_get_string_value_wrong_type() {
        let client = RParifClient::new("api-key");
        let data = object! {
            key: 12
        };

        let result = client.get_string_value("key", &data);

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::WrongJsonType { expected, json } => {
                assert_eq!(expected, "string".to_string());
                assert_eq!(json, "12")
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    // Get string value from JSON ok
    fn test_get_string_value() {
        let client = RParifClient::new("api-key");
        let data = object! {
            key: "data"
        };

        let result = client.get_string_value("key", &data);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some("data"));
    }

    #[test]
    // Call return an error because reqwest return an error
    fn test_execute_query_reqwest_error() {
        let client = RParifClient::new("api-key");
        let result = client.execute_query("http://localhost:5001");

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::RequestError(err) => {
                assert_eq!(
                    err.url(),
                    Some(&Url::parse("http://localhost:5001").unwrap())
                );
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    #[with_mock_server]
    // Call return an error because the result is not a well formed JSON
    fn test_execute_query_reqwest_not_json() {
        let _search_mock = mock(GET, "/path")
            .return_status(200)
            .return_body("this is not a json")
            .create();

        let client = RParifClient::new("api-key");
        let result = client.execute_query("http://localhost:5000/path");

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::JsonError(err) => {
                assert_eq!(format!("{}", err), "Unexpected character: h at (1:2)")
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    #[with_mock_server]
    // Call return an error because status is different from 2xx
    fn test_execute_query_reqwest_wrong_status() {
        let _search_mock = mock(GET, "/path")
            .return_status(300)
            .return_body("{\"data\":0}")
            .create();

        let client = RParifClient::new("api-key");
        let result = client.execute_query("http://localhost:5000/path");

        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::CallError { url, body, status } => {
                assert_eq!(url, "http://localhost:5000/path".to_string());
                assert_eq!(body, "{\"data\":0}".to_string());
                assert_eq!(status, 300);
            }
            _ => panic!("Wrong error"),
        }
    }

    #[test]
    #[with_mock_server]
    // Call OK
    fn test_execute_query_reqwest() {
        let _search_mock = mock(GET, "/path")
            .return_status(200)
            .return_body("{\"data\":0}")
            .create();

        let client = RParifClient::new("api-key");
        let result = client.execute_query("http://localhost:5000/path");

        assert_eq!(
            result.ok(),
            Some(object! {
                data: 0
            })
        )
    }

    #[test]
    fn test_index_to_index() {
        let client = RParifClient::new("api-key");
        let data = array![{
               date: "jour",
               indice: 35,
               url_carte: "a"
        }];

        let result = client.index_to_index(data);

        assert!(result.is_ok());
        assert_eq!(
            result.ok(),
            Some(vec![Index::new(
                Utc::today().naive_utc(),
                Some("a".to_string()),
                vec!["global".to_string()],
                35,
                None,
            )])
        );
    }

    #[test]
    fn test_index_to_index_not_an_array() {
        let client = RParifClient::new("api-key");
        let data = object! {
               date: "jour",
               indice: 35,
               url_carte: "a"
        };

        let result = client.index_to_index(data);
        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::WrongJsonType { expected, json } => {
                assert_eq!(expected, "array".to_string());
                assert_eq!(
                    json,
                    r#"{"date":"jour","indice":35,"url_carte":"a"}"#.to_string()
                )
            }
            _ => panic!("Wrong error"),
        };
    }

    #[test]
    fn test_index_day_to_index() {
        let client = RParifClient::new("api-key");
        let data = object! {
            date: "31/12/2019",
            global: object! {
                indice: 35,
                url_carte: "a"
            },
            o3: object! {
                indice: 40,
                url_carte: "b"
            },
        };
        let expected = Some(vec![
            Index::new(
                NaiveDate::from_ymd(2019, 12, 31),
                Some("a".to_string()),
                vec!["global".to_string()],
                35,
                None,
            ),
            Index::new(
                NaiveDate::from_ymd(2019, 12, 31),
                Some("b".to_string()),
                vec!["o3".to_string()],
                40,
                None,
            ),
        ]);

        let result = client.index_day_to_index(data);
        assert!(result.is_ok());
        assert_eq!(result.ok(), expected);
    }

    #[test]
    fn test_idxville_to_index() {
        let client = RParifClient::new("api-key");
        let data = array![
            {
                ninsee: "75101",
                hier: {
                    indice: 25,
                    polluants: ["no2", "pm10"]
                },
                jour: {
                    indice: 50,
                    polluants: ["pm10"]
                },
                demain: {
                    indice: 36,
                    polluants: ["o3"]
                },
            },
            {
                ninsee: "94028",
                hier: {
                    indice: 100,
                    polluants: ["no2"]
                },
                jour: {
                    indice: 40,
                    polluants: ["o3"]
                },
                demain: {
                    indice: 95,
                    polluants: ["o3","no2","pm10"]
                },
            }
        ];

        let result = client.idxville_to_index(data);

        let today = Utc::today();
        let yesterday = today.checked_sub_signed(Duration::days(1)).unwrap();
        let tomorrow = today.checked_add_signed(Duration::days(1)).unwrap();
        let expected = vec![
            Index::new(
                NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day())
                    .unwrap(),
                None,
                vec!["no2".to_string(), "pm10".to_string()],
                25,
                Some("75101".to_string()),
            ),
            Index::new(
                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
                None,
                vec!["pm10".to_string()],
                50,
                Some("75101".to_string()),
            ),
            Index::new(
                NaiveDate::from_ymd_opt(tomorrow.year(), tomorrow.month(), tomorrow.day()).unwrap(),
                None,
                vec!["o3".to_string()],
                36,
                Some("75101".to_string()),
            ),
            Index::new(
                NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day())
                    .unwrap(),
                None,
                vec!["no2".to_string()],
                100,
                Some("94028".to_string()),
            ),
            Index::new(
                NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
                None,
                vec!["o3".to_string()],
                40,
                Some("94028".to_string()),
            ),
            Index::new(
                NaiveDate::from_ymd_opt(tomorrow.year(), tomorrow.month(), tomorrow.day()).unwrap(),
                None,
                vec!["o3".to_string(), "no2".to_string(), "pm10".to_string()],
                95,
                Some("94028".to_string()),
            ),
        ];

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(expected));
    }

    #[test]
    fn test_idxville_to_index_not_an_array() {
        let client = RParifClient::new("api-key");
        let data = object! {
               date: "jour",
               indice: 35,
               url_carte: "a"
        };

        let result = client.idxville_to_index(data);
        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::WrongJsonType { expected, json } => {
                assert_eq!(expected, "array".to_string());
                assert_eq!(
                    json,
                    r#"{"date":"jour","indice":35,"url_carte":"a"}"#.to_string()
                )
            }
            _ => panic!("Wrong error"),
        };
    }

    #[test]
    fn test_episode_to_episode() {
        let client = RParifClient::new("api-key");
        let data = array![
             {
                date: "hier",
                detail: "",
                o3: {
                    type: "constate",
                    niveau: "info",
                    criteres: ["km","pop"]
                },
                so2: {
                    type: "constate",
                    niveau: "alerte",
                    criteres: ["pop"]

                }
             },
             {
                date: "jour",
                detail: "Il est conseillé d'éviter les déplacements en Ile de France",
                no2: {
                    type: "constate",
                    niveau: "normal",
                    criteres: ["km"]
                },
                so2: {
                    type: "constate",
                    niveau: "alerte",
                    criteres: ["km"]
                }
             },
             {
                date: "demain",
                detail:""
             }
        ];

        let result = client.episode_to_episode(data);

        let today = Utc::today();
        let yesterday = today.checked_sub_signed(Duration::days(1)).unwrap();
        let tomorrow = today.checked_add_signed(Duration::days(1)).unwrap();
        let mut expected = Vec::new();
        let mut episode = Episode::new(
            NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day()).unwrap(),
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
        expected.push(episode);
        let mut episode = Episode::new(
            NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
            Some("Il est conseillé d'éviter les déplacements en Ile de France".to_string()),
        );
        episode.add(
            "no2".to_string(),
            Type::Observed,
            Level::Normal,
            vec![Criteria::Area],
        );
        episode.add(
            "so2".to_string(),
            Type::Observed,
            Level::Alert,
            vec![Criteria::Area],
        );
        expected.push(episode);
        let episode = Episode::new(
            NaiveDate::from_ymd_opt(tomorrow.year(), tomorrow.month(), tomorrow.day()).unwrap(),
            None,
        );
        expected.push(episode);

        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(expected));
    }

    #[test]
    fn test_episode_to_episode_not_an_array() {
        let client = RParifClient::new("api-key");
        let data = object! {
        date: "jour",
        indice: 35,
        url_carte: "a"
        };

        let result = client.episode_to_episode(data);
        assert!(result.is_err());
        match result.err().unwrap() {
            RParifError::WrongJsonType { expected, json } => {
                assert_eq!(expected, "array".to_string());
                assert_eq!(
                    json,
                    r#"{"date":"jour","indice":35,"url_carte":"a"}"#.to_string()
                )
            }
            _ => panic!("Wrong error"),
        };
    }
}
