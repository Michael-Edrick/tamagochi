#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tamagotchi {
    weight: u32,
    hungry_meter: u32,
    happiness_meter: u32,
    is_sick: bool,
}

const PET: Symbol = symbol_short!("PET");

#[contract]
pub struct TamagotchiContract;

#[contractimpl]
impl TamagotchiContract { 
    pub fn feed(env: Env, meal_type: String) -> Tamagotchi {
        // Get current state
        let mut tamagotchi = Self::get_state(env.clone());

        // Check if the Tamagotchi is sick
        if tamagotchi.is_sick {
            return tamagotchi; // If sick, do nothing
        }

        let meal = soroban_sdk::String::from_str(&env , "meal");
        let snack = soroban_sdk::String::from_str(&env ,"snack");

        if meal_type == meal {
            tamagotchi.hungry_meter += 4;
            tamagotchi.weight += 2;
        } else if meal_type == snack {
            tamagotchi.hungry_meter += 1;
            tamagotchi.weight += 1;
        } else {
            panic!("Invalid meal type")
        }

        // Check if overfeeding
        if tamagotchi.hungry_meter > 4 {
            tamagotchi.is_sick = true;
            tamagotchi.hungry_meter = 4;
        }

        // Save the update
        env.storage().instance().set(&PET, &tamagotchi);
        
        tamagotchi 

    }

    pub fn play(env: Env, guess: String) -> String {
        // Get current state
        let mut tamagotchi = Self::get_state(env.clone());

        //initial value of result
        let mut result = soroban_sdk::String::from_str(&env ,"sick");

        // Check if the Tamagotchi is sick
        if tamagotchi.is_sick {
            return result; // If sick, do nothing
        }

        let options = ["left", "right"];
        let random_index = Self::prng_u64_in_range(env.clone(), 0, 1) as usize;
        let tamagotchi_facing = options[random_index];

        result = soroban_sdk::String::from_str(&env ,"lose");

        match guess {
            tamagotchi_facing => {
                // Player guessed correctly
                tamagotchi.happiness_meter = (tamagotchi.happiness_meter + 1).min(4); // Ensure happiness_meter doesn't exceed 4
                result = soroban_sdk::String::from_str(&env ,"win");

            }
        }

        tamagotchi.weight = tamagotchi.weight.saturating_sub(1);
        tamagotchi.hungry_meter = tamagotchi.hungry_meter.saturating_sub(1);

        // Save the update
        env.storage().instance().set(&PET, &tamagotchi);

        result
    }

    pub fn medicine(env: Env) -> Tamagotchi {
        // Get current state
        let mut tamagotchi = Self::get_state(env.clone());

        // Check is sick status
        if tamagotchi.is_sick {
            tamagotchi.is_sick = false;
        }

        // Save the update
        env.storage().instance().set(&PET, &tamagotchi);

        tamagotchi
    }

    /// Return the current state.
    pub fn get_state(env: Env) -> Tamagotchi {
        env.storage().instance().get(&PET).unwrap_or(Tamagotchi {
            weight: 0,
            hungry_meter: 0,
            happiness_meter: 0,
            is_sick: false,
        }) // If no value set, assume default
    }

    pub fn prng_u64_in_range(env: Env, low: u64, high: u64) -> u64 {
        env.prng().gen_range(low..=high)
    }
}

#[cfg(test)]
mod test;