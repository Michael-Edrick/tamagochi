#![cfg(test)]
// test.rs

use super::*;
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TamagotchiContract);
    let client = TamagotchiContractClient::new(&env, &contract_id);

    let left = soroban_sdk::String::from_str(&env , "left");
    let right = soroban_sdk::String::from_str(&env ,"right");

    let meal = soroban_sdk::String::from_str(&env , "meal");
    let snack = soroban_sdk::String::from_str(&env ,"snack");

    // Test feeding a snack
    assert_eq!(
        client.feed(&snack),
        Tamagotchi {
            weight: 1,
            hungry_meter: 1,
            happiness_meter: 0,
            is_sick: false
        }
    );

    // Test Get State after feeding
    assert_eq!(
        client.getstate(),
        Tamagotchi {
            weight: 1,
            hungry_meter: 1,
            happiness_meter: 0,
            is_sick: false
        }
    );

    // Test feeding meal until Tamagotchi is sick (no change expected)
    assert_eq!(
        client.feed(&meal),
        Tamagotchi {
            weight: 3,
            hungry_meter: 4,
            happiness_meter: 0,
            is_sick: true
        }
    );

    // Test giving medicine to a sick Tamagotchi
    assert_eq!(
        client.medicine(),
        Tamagotchi {
            weight: 3,
            hungry_meter: 4,
            happiness_meter: 0,
            is_sick: false
        }
    );

    // Test Get State after giving medicine
    assert_eq!(
        client.getstate(),
        Tamagotchi {
            weight: 3,
            hungry_meter: 4,
            happiness_meter: 0,
            is_sick: false
        }
    );
}