#[cfg(test)]
extern crate env_logger;
extern crate httpmock;
extern crate rparif;

use chrono::{Datelike, Duration, NaiveDate, Utc};
use httpmock::Method::GET;
use httpmock::prelude::*;

use rparif::client::RParifClient;
use rparif::objects::{Criteria, Day, Episode, Index, Level, Type};

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_indice() {
    init();

    let server = MockServer::start();
    let search_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/indice")
            .query_param("key", "dummy");
        then.status(200)
            .body("[{\"date\":\"hier\",\"indice\":35,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier\"},\
            {\"date\":\"jour\",\"indice\":50,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/jour\"},\
            {\"date\":\"demain\",\"indice\":70,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/demain\"}]");
    });

    let base_url = &server.base_url();
    let client = RParifClient::new_test("dummy", base_url);
    let result = client.index();

    search_mock.assert();
    assert!(result.is_ok(), "Got an Err() : {:?}", result);
    let today = Utc::today();
    let yesterday = today.checked_sub_signed(Duration::days(1)).unwrap();
    let tomorrow = today.checked_add_signed(Duration::days(1)).unwrap();
    let mut expected = Vec::new();
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day()).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/hier".to_string()),
        vec!["global".to_string()],
        35,
        None,
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/jour".to_string()),
        vec!["global".to_string()],
        50,
        None,
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(tomorrow.year(), tomorrow.month(), tomorrow.day()).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/demain".to_string()),
        vec!["global".to_string()],
        70,
        None,
    ));

    assert_eq!(result.ok(), Some(expected));
}

#[test]
fn test_indice_day() {
    init();

    let server = MockServer::start();
    let search_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/indiceJour")
            .query_param("key", "dummy")
            .query_param("date", "jour");
        then.status(200)
            .body("{\"date\":\"09/08/2012\",\"global\":{\"indice\":35,\"url_carte\":\
            \"http://localhost:5000/services/cartes/indice/date/hier\"},\"o2\":{\"indice\":20,\
            \"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier/pol/O2\"},\
            \"o3\":{\"indice\":86,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier/pol/O3\"},\
            \"pm10\":{\"indice\":125,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier/pol/PM10\"}}");
    });

    let base_url = &server.base_url();
    let client = RParifClient::new_test("dummy", base_url);
    let result = client.index_day(Day::Today);

    search_mock.assert();
    assert!(result.is_ok(), "Got an Err() : {:?}", result);

    let mut expected = Vec::new();
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(2012, 8, 9).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/hier".to_string()),
        vec!["global".to_string()],
        35,
        None,
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(2012, 8, 9).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/hier/pol/O2".to_string()),
        vec!["o2".to_string()],
        20,
        None,
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(2012, 8, 9).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/hier/pol/O3".to_string()),
        vec!["o3".to_string()],
        86,
        None,
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(2012, 8, 9).unwrap(),
        Some("http://localhost:5000/services/cartes/indice/date/hier/pol/PM10".to_string()),
        vec!["pm10".to_string()],
        125,
        None,
    ));

    assert_eq!(result.ok(), Some(expected));
}

#[test]
fn test_indice_city() {
    init();

    let server = MockServer::start();
    let search_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/idxville")
            .query_param("key", "dummy")
            .query_param("villes", "75120,94038");
        then.status(200)
            .body("[{\"ninsee\":\"75101\",\"hier\":{\"indice\":25,\"polluants\":[\"no2\",\"pm10\"]},\"jour\":\
            {\"indice\":50,\"polluants\":[\"pm10\"]},\"demain\":{\"indice\":36,\"polluants\":[\"o3\"]}},{\"ninsee\":\"94028\",\
            \"hier\":{\"indice\":100,\"polluants\":[\"no2\"]},\"jour\":{\"indice\":40,\"polluants\":[\"o3\"]},\"demain\":\
            {\"indice\":95,\"polluants\":[\"o3\",\"no2\",\"pm10\"]}}]");
    });

    let base_url = &server.base_url();
    let client = RParifClient::new_test("dummy", base_url);
    let result = client.index_city(vec!["75120", "94038"]);

    search_mock.assert();
    assert!(result.is_ok(), "Got an Err() : {:?}", result);

    let today = Utc::today();
    let yesterday = today.checked_sub_signed(Duration::days(1)).unwrap();
    let tomorrow = today.checked_add_signed(Duration::days(1)).unwrap();

    let expected = vec![
        Index::new(
            NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day()).unwrap(),
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
            NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day()).unwrap(),
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

    assert_eq!(result.ok(), Some(expected));
}

#[test]
fn test_episode() {
    init();

    let server = MockServer::start();
    let search_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/episode")
            .query_param("key", "dummy");
        then.status(200)
            .body("[{\"date\":\"hier\",\"o3\":{\"type\":\"constate\",\"niveau\":\"info\",\"criteres\":\
            [\"km\",\"pop\"]},\"so2\":{\"type\":\"constate\",\"niveau\":\"alerte\",\"criteres\":[\"pop\"]},\"detail\":\"\"},\
            {\"date\":\"jour\",\"no2\":{\"type\":\"constate\",\"niveau\":\"normal\",\"criteres\":[\"km\"]},\"so2\":{\"type\":\
            \"constate\",\"niveau\":\"alerte\",\"criteres\":[\"km\"]},\"detail\":\"Il est conseillé d'éviter les déplacements en Ile de France\"},\
            {\"date\":\"demain\",\"detail\":\"\"}]");
    });

    let base_url = &server.base_url();
    let client = RParifClient::new_test("dummy", base_url);
    let result = client.episode();

    search_mock.assert();
    assert!(result.is_ok(), "Got an Err() : {:?}", result);

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

    assert_eq!(result.ok(), Some(expected));
}
