#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur pilihan dalam poll
#[contracttype]
#[derive(Clone, Debug)]
pub struct OptionItem {
    name: String,
    votes: u32,
}

// Struktur Poll
#[contracttype]
#[derive(Clone, Debug)]
pub struct Poll {
    id: u64,
    question: String,
    options: Vec<OptionItem>,
}

// Storage key
const POLL_DATA: Symbol = symbol_short!("POLL_DATA");

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Ambil semua poll
    pub fn get_polls(env: Env) -> Vec<Poll> {
        env.storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Buat poll baru
    pub fn create_poll(env: Env, question: String, option_names: Vec<String>) -> String {
        let mut polls: Vec<Poll> = env.storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        let mut options: Vec<OptionItem> = Vec::new(&env);

        for i in 0..option_names.len() {
            options.push_back(OptionItem {
                name: option_names.get(i).unwrap(),
                votes: 0,
            });
        }

        let poll = Poll {
            id: env.prng().gen::<u64>(),
            question,
            options,
        };

        polls.push_back(poll);
        env.storage().instance().set(&POLL_DATA, &polls);

        String::from_str(&env, "Poll berhasil dibuat")
    }

    // Vote ke salah satu pilihan
    pub fn vote(env: Env, poll_id: u64, option_index: u32) -> String {
        let mut polls: Vec<Poll> = env.storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..polls.len() {
            let mut poll = polls.get(i).unwrap();

            if poll.id == poll_id {
                if option_index >= poll.options.len() {
                    return String::from_str(&env, "Pilihan tidak valid");
                }

                let mut option = poll.options.get(option_index).unwrap();
                option.votes += 1;

                poll.options.set(option_index, option);
                polls.set(i, poll);

                env.storage().instance().set(&POLL_DATA, &polls);
                return String::from_str(&env, "Vote berhasil");
            }
        }

        String::from_str(&env, "Poll tidak ditemukan")
    }

    // Hapus poll
    pub fn delete_poll(env: Env, id: u64) -> String {
        let mut polls: Vec<Poll> = env.storage()
            .instance()
            .get(&POLL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..polls.len() {
            if polls.get(i).unwrap().id == id {
                polls.remove(i);
                env.storage().instance().set(&POLL_DATA, &polls);
                return String::from_str(&env, "Poll dihapus");
            }
        }

        String::from_str(&env, "Poll tidak ditemukan")
    }
}

mod test;