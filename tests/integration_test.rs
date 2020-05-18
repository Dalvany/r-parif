#[cfg(test)]
extern crate env_logger;
extern crate httpmock;
extern crate rparif;

use chrono::{Datelike, Duration, NaiveDate, Utc};
use httpmock::Method::GET;
use httpmock::{mock, with_mock_server};

use rparif::client::RParifClient;
use rparif::objects::{Criteria, Day, Episode, Index, Level, Type};

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
#[with_mock_server]
fn test_indice() {
    init();
    let search_mock = mock(GET, "/indice")
        .expect_query_param("key", "dummy")
        .return_status(200)
        .return_body("[{\"date\":\"hier\",\"indice\":35,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier\"},\
            {\"date\":\"jour\",\"indice\":50,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/jour\"},\
            {\"date\":\"demain\",\"indice\":70,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/demain\"}]")
        .create();

    let client = RParifClient::new_test("http://localhost:5000");
    let result = client.index();

    assert_eq!(search_mock.times_called(), 1);
    assert!(result.is_ok(), format!("Got an Err() : {:?}", result));
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
#[with_mock_server]
fn test_indice_day() {
    init();
    let search_mock = mock(GET, "/indiceJour")
        .expect_query_param("key", "dummy")
        .expect_query_param("date", "jour")
        .return_status(200)
        .return_body("{\"date\":\"09/08/2012\",\"global\":{\"indice\":35,\"url_carte\":\
            \"http://localhost:5000/services/cartes/indice/date/hier\"},\"o2\":{\"indice\":20,\
            \"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier/pol/O2\"},\
            \"o3\":{\"indice\":86,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier/pol/O3\"},\
            \"pm10\":{\"indice\":125,\"url_carte\":\"http://localhost:5000/services/cartes/indice/date/hier/pol/PM10\"}}")
        .create();

    let client = RParifClient::new_test("http://localhost:5000");
    let result = client.index_day(Day::Today);

    assert_eq!(search_mock.times_called(), 1);
    assert!(result.is_ok(), format!("Got an Err() : {:?}", result));

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
#[with_mock_server]
fn test_indice_city() {
    init();
    let search_mock = mock(GET, "/idxville")
        .expect_query_param("key", "dummy")
        .expect_query_param("villes", "75120,94038")
        .return_status(200)
        .return_body("[{\"ninsee\":\"75101\",\"hier\":{\"indice\":25,\"polluants\":[\"no2\",\"pm10\"]},\"jour\":\
            {\"indice\":50,\"polluants\":[\"pm10\"]},\"demain\":{\"indice\":36,\"polluants\":[\"o3\"]}},{\"ninsee\":\"94028\",\
            \"hier\":{\"indice\":100,\"polluants\":[\"no2\"]},\"jour\":{\"indice\":40,\"polluants\":[\"o3\"]},\"demain\":\
            {\"indice\":95,\"polluants\":[\"o3\",\"no2\",\"pm10\"]}}]")
        .create();

    let client = RParifClient::new_test("http://localhost:5000");
    let result = client.index_city(vec!["75120", "94038"]);

    assert_eq!(search_mock.times_called(), 1);
    assert!(result.is_ok(), format!("Got an Err() : {:?}", result));

    let today = Utc::today();
    let yesterday = today.checked_sub_signed(Duration::days(1)).unwrap();
    let tomorrow = today.checked_add_signed(Duration::days(1)).unwrap();

    let mut expected = Vec::new();
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day()).unwrap(),
        None,
        vec!["no2".to_string(), "pm10".to_string()],
        25,
        Some("75101".to_string()),
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
        None,
        vec!["pm10".to_string()],
        50,
        Some("75101".to_string()),
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(tomorrow.year(), tomorrow.month(), tomorrow.day()).unwrap(),
        None,
        vec!["o3".to_string()],
        36,
        Some("75101".to_string()),
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(yesterday.year(), yesterday.month(), yesterday.day()).unwrap(),
        None,
        vec!["no2".to_string()],
        100,
        Some("94028".to_string()),
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap(),
        None,
        vec!["o3".to_string()],
        40,
        Some("94028".to_string()),
    ));
    expected.push(Index::new(
        NaiveDate::from_ymd_opt(tomorrow.year(), tomorrow.month(), tomorrow.day()).unwrap(),
        None,
        vec!["o3".to_string(), "no2".to_string(), "pm10".to_string()],
        95,
        Some("94028".to_string()),
    ));

    assert_eq!(result.ok(), Some(expected));
}

#[test]
#[with_mock_server]
fn test_episode() {
    init();
    let search_mock = mock(GET, "/episode")
        .expect_query_param("key", "dummy")
        .return_status(200)
        .return_body("[{\"date\":\"hier\",\"o3\":{\"type\":\"constate\",\"niveau\":\"info\",\"criteres\":\
            [\"km\",\"pop\"]},\"so2\":{\"type\":\"constate\",\"niveau\":\"alerte\",\"criteres\":[\"pop\"]},\"detail\":\"\"},\
            {\"date\":\"jour\",\"no2\":{\"type\":\"constate\",\"niveau\":\"normal\",\"criteres\":[\"km\"]},\"so2\":{\"type\":\
            \"constate\",\"niveau\":\"alerte\",\"criteres\":[\"km\"]},\"detail\":\"Il est conseillé d'éviter les déplacements en Ile de France\"},\
            {\"date\":\"demain\",\"detail\":\"\"}]")
        .create();

    let client = RParifClient::new_test("http://localhost:5000");
    let result = client.episode();

    assert_eq!(search_mock.times_called(), 1);
    assert!(result.is_ok(), format!("Got an Err() : {:?}", result));

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

#[test]
#[with_mock_server]
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
