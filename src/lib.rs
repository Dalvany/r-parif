//! This lib allow to access [Airparif](https://www.airparif.asso.fr/) indice pollution API.
//! It needs an [API key](https://www.airparif.asso.fr/rss/api) to work.
#[macro_use]
extern crate log;

use crate::client::RParifClient;
use crate::error::RParifError;
use crate::objects::{Day, Episode, Index};

pub mod client;
pub mod error;
pub mod objects;

/// Convenient function that allow easy to access [`indice`](./client/struct.RParifClient.html#method.indice) endpoint.  
/// If multiple calls needs to be made to HTTP API, use [RParifClient](./client/struct.RParifClient.html)
///
/// # Arguments
///
/// * `api_key` - API key
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
/// JSON is missing `indice` pollution
///
/// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `indice`
/// pollution is not a number or if JSON is not as expected
///
/// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
/// formed JSON
pub fn indice(api_key: &str) -> Result<Vec<Index>, RParifError> {
    RParifClient::new(api_key).index()
}

/// Convenient function that allow easy to access [`indiceJour`](./client/struct.RParifClient.html#method.indice_day) endpoint.  
/// If multiple calls needs to be made to HTTP API, use [RParifClient](./client/struct.RParifClient.html)
///
/// # Arguments
///
/// * `api_key` - API key
///
/// * `day` - Which day to get pollution indices for
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
/// JSON is missing `indice` pollution or `date`
///
/// * [RParifError::WrongJsonType](../error/enum.RParifError.html#variant.WrongJsonType) if `indice`
/// pollution is not a number or if `date` isn't a string
///
/// * [RParifError::DateParseError](../error/enum.RParifError.html#variant.DateParseError) if `date`
/// is not in `dd/mm/yyyy` format
///
/// * [RParifError::JsonError](../error/enum.RParifError.html#variant.JsonError) if response isn't a well
/// formed JSON
pub fn indice_day(api_key: &str, day: Day) -> Result<Vec<Index>, RParifError> {
    RParifClient::new(api_key).index_day(day)
}

/// Convenient function that allow easy to access [`idxville`](./client/struct.RParifClient.html#method.indice_city) endpoint.  
/// If multiple calls needs to be made to HTTP API, use [RParifClient](./client/struct.RParifClient.html)
///
/// # Arguments
///
/// * `api_key` - API key
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
pub fn indice_city(api_key: &str, cities: Vec<&str>) -> Result<Vec<Index>, RParifError> {
    RParifClient::new(api_key).index_city(cities)
}

/// Convenient function that allow easy to access [`episode`](./client/struct.RParifClient.html#method.episode) endpoint.  
/// If multiple calls needs to be made to HTTP API, use [RParifClient](./client/struct.RParifClient.html)
///
/// # Arguments
///
/// * `api_key` - API key
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
pub fn episode(api_key: &str) -> Result<Vec<Episode>, RParifError> {
    RParifClient::new(api_key).episode()
}
