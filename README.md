# R-Parif

A Rust library that allows access [Airparif](https://www.airparif.asso.fr/) indice 
pollution API for Ile-de-France.  
It needs an [API key](https://www.airparif.asso.fr/rss/api) to work.

# API

It allows access to :
* indice : it provides the global pollution index
* indiceJour : it returns global and per pollutant indices for previous, current or next day
* idxville : returns indice and pollutant for given cities for previous, current and next day
* episode : returns pollution alerts

# Serde

With serde feature, data structures implemente Serde's `Serialize` and `Deserialize`

# Examples

```rust,no_run
extern crate rparif;

use rparif::client::RParifClient;
use rparif::error::RParifError;

fn main() -> Result<(), RParifError> {
    let client = RParifClient::new("my-api-key");
    let indices = client.index()?;
    for index in indices.into_iter() {
        println!("{}", index);
    }
    Ok(())
}
```
With a valid API key :
```
2020-05-17 (city : None) : ["global"] = 53 (map : Some("https://www.airparif.asso.fr/services/cartes/indice/date/hier"))
2020-05-18 (city : None) : ["global"] = 49 (map : Some("https://www.airparif.asso.fr/services/cartes/indice/date/jour"))
2020-05-19 (city : None) : ["global"] = 49 (map : Some("https://www.airparif.asso.fr/services/cartes/indice/date/demain"))
```
With an invalid API key :
```
Error : Some(CallError { url: "https://www.airparif.asso.fr/services/api/1.1/indice?key=wrong-api", body: "{\"erreur\":\"Cl\\u00e9 invalide\"}", status: 403 })
```