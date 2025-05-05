use std::os::macos::raw::stat;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("some custom code...");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawai,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Luisiana,
    Maine,
    Maryland,
    Massachusetts,
    Míchigan,
    Minnesota,
    Misisipi,
    Misuri,
    Montana,
    Nebraska,
    Nevada,
    NuevoHampshire,
    NuevaJersey,
    NuevoMexico,
    NuevaYork,
    CarolinaDelNorte,
    DakotaDelNorte,
    Ohio,
    Oklahoma,
    Oregón,
    Pensilvania,
    RhodeIsland,
    CarolinaDelSur,
    DakotaDelSur,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    VirginiaOccidental,
    Wisconsin,
    Wyoming,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            _ => false,
        }
    }
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Virginia);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }

    let some_state = UsState::Alabama;
    println!("State exists in 1820: {}", some_state.existed_in(1810));

    if let UsState::Alabama = some_state {
        println!("Is Alabama");
    }

    let alaska_coin = Coin::Quarter(UsState::Alaska);
    if let Some(description) = describe_state_quarter(alaska_coin) {
        println!("Description: {}", description);
    }
    // println!("State: {}", describe_state_quarter(alaska_coin));
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is pretty new"))
        }
    } else {
        None
    }
}

/* It works but it is very annoying ... */
fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter3(coin: Coin) -> Option<String> {
    // dejamos state en el scope si es que es un Coin::Quarter..., si no (else), regresamos inmediatamente None
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}