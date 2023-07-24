//! The automated teller machine gives you cash after you swipe your card and enter your pin.
//! The atm may fail to give you cash if it is empty or you haven't swiped your card, or you have
//! entered the wrong pin.

use crate::traits::StateMachine;
use crate::traits::hash;


use std::fmt::Formatter;
//use crate::traits::hash;
//use std::ptr::hash;

/// The keys on the ATM keypad
#[derive(Hash)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]

pub enum Key {
    One,
    Two,
    Three,
    Four,
    Enter,
}




/// Something you can do to the ATM
#[derive(Debug)]
#[derive(PartialEq)]

pub enum Action {
    /// Swipe your card at the ATM. The attached value is the hash of the pin
    /// that should be keyed in on the keypad next.
    SwipeCard(u64),
    /// Press a key on the keypad
    PressKey(Key),
}

/// The various states of authentication possible with the ATM
#[derive(Debug)]
#[derive(PartialEq)]

enum Auth {
    /// No session has begun yet. Waiting for the user to swipe their card
    Waiting,
    /// The user has swiped their card, providing the enclosed PIN hash.
    /// Waiting for the user to key in their pin
    Authenticating(u64),
    /// The user has authenticated. Waiting for them to key in the amount
    /// of cash to withdraw
    Authenticated,
}



/// The ATM. When a card is swiped, the ATM learns the correct pin's hash.
/// It waits for you to key in your pin. You can press as many numeric keys as
/// you like followed by enter. If the pin is incorrect, your card is returned
/// and the ATM automatically goes back to the main menu. If your pin is correct,
/// the ATM waits for you to key in an amount of money to withdraw. Withdraws
/// are bounded only by the cash in the machine (there is no account balance).
#[derive(Debug)]
#[derive(PartialEq)]

pub struct Atm {
    /// How much money is in the ATM
    cash_inside: u64,
    /// The machine's authentication status.
    expected_pin_hash: Auth,
    /// All the keys that have been pressed since the last `Enter`
    keystroke_register:Vec<Key>,
}


//TODO
// Implement trait Default for Auth 
// return Waiting status 
impl Default for Auth {
    fn default() -> Self {
        Auth::Waiting
    }
}


//TODO
// Implement trait From  for &str
// Convert  elements in Key to &str
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Key::One => write!(f, "1"),
            Key::Two => write!(f, "2"),
            Key::Three => write!(f, "3"),
            Key::Four => write!(f, "4"),
            Key::Enter => write!(f, "0"),
        }
    }
    
}
impl From<Key> for &str {
    fn from(value: Key) -> Self {
       // let v = value.clone().to_string().as_str();
       // Box::leak(v.into_boxed_str())
        let str_value = value.to_string(); // Convert u32 to String
    //let st = String::from(str_value);
    //let str_ref: &'static str =  &str_value;
        Box::leak(str_value.into_boxed_str())
    }
}

fn return_a_key(k:&Key) -> Key
{
    match k {
        Key::One => Key::One,
        Key::Two => Key::Two,
        Key::Three => Key::Three,
        Key::Four => Key::Four,
        Key::Enter => Key::Enter,
    }
}

impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
    type State = Atm;
    type Transition = Action;
    // Hint
    // Should use `default` method when auth status is Waiting status
    // Should use `from` method to convert  elements in Key to &str
    // Parse &str to integer to calculate amount
    // Use a hash function to verify the PIN both before and after the user presses the Enter key.
    fn next_state(starting_state:  &mut Self::State, t: &Self::Transition) -> Self::State {
        match starting_state.expected_pin_hash {
            Auth::Waiting => {
                match t {
                    Action::SwipeCard(num) => 
                    {
                        Atm {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticating(num.clone()),
                            keystroke_register: Vec::new(),
                        }
                    },
                    _=> {
                        Atm {
                            // tra ve default
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Waiting,
                            keystroke_register: Vec::new(),
                        }
                    }
                }
            }
            Auth::Authenticating(num) => {
                match t {
                    Action::SwipeCard(num) => 
                    {
                        Atm {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticating(num.clone()),
                            keystroke_register: starting_state.keystroke_register.clone(),
                        }
                    },
                    Action::PressKey(k) =>
                    {
                        match k  {
                            Key::Enter =>
                            {
                                let hash = hash(&starting_state.keystroke_register);
                                if (hash) == num {
                                    return Atm {
                                        cash_inside: starting_state.cash_inside,
                                        expected_pin_hash: Auth::Authenticated,
                                        keystroke_register: Vec::new(),
                                    }
                                }
                                else {
                                    return Atm {
                                        cash_inside: starting_state.cash_inside,
                                        expected_pin_hash: Auth::Waiting,
                                        keystroke_register: Vec::new()
                                    }
                                }
                            },
                            _ => {
                                starting_state.keystroke_register.push(return_a_key(k));

                                return Atm {
                                    cash_inside: starting_state.cash_inside,
                                    expected_pin_hash: Auth::Authenticating(num.clone()),
                                    keystroke_register: starting_state.keystroke_register.clone()
                                }
                            }
                        }     
                    }
                }
            },
            Auth::Authenticated => 
            {
               match t {
                   Action::PressKey(k) =>
                   {
                    match k {
                        Key::Enter =>

                        {
                        //    let ss = vec!["a", "b", "c"].iter().fold("".to_string(), |cur: String, nxt: &&str| cur + nxt);
                           // let v = starting_state.keystroke_register.into_iter();
                            let s =  String::from(starting_state.keystroke_register.iter().map(|a| a.to_string()).collect::<Vec<String>>().iter().fold(String::new(), |mut cur: String, st:&String| {cur.push_str(st.clone().as_str()) ; return cur}));

                            //let s =  String::from(v.fold(String::new(), |a:Key, b : Key| a.to_string() + b.to_string().as_str()));
                            let num =  s.parse::<u64>().unwrap();
                            if num > starting_state.cash_inside
                            {
                                Atm {
                                    cash_inside: starting_state.cash_inside,
                                    expected_pin_hash: Auth::Waiting,
                                    keystroke_register: Vec::new()
                                }
                            }
                            else {
                                Atm {
                                    cash_inside: starting_state.cash_inside - num,
                                    expected_pin_hash: Auth::Waiting,
                                    keystroke_register: Vec::new()
                                }
                            }
                        },
                        _=>
                        {
                            starting_state.keystroke_register.push(return_a_key(k));
                            Atm {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Authenticated,
                                keystroke_register: starting_state.keystroke_register.clone(),
                            }
                        }
                    }
                   }
                   , _ => {
                    //tra ve default
                    Atm {
                        cash_inside: starting_state.cash_inside,
                        expected_pin_hash: Auth::Waiting,
                        keystroke_register: Vec::new(),
                    }
                   } 
               }
            },
        }
    }
}

#[test]
fn sm_3_simple_swipe_card() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let  end = Atm::next_state(&mut start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_swipe_card_again_part_way_through() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&mut start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);

    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };
    let end = Atm::next_state(&mut start, &Action::SwipeCard(1234));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Three],
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_press_key_before_card_swipe() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_pin() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&mut start, &Action::PressKey(Key::Two));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(1234),
        keystroke_register: vec![Key::One, Key::Two],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_enter_wrong_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = hash(&pin);

    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_correct_pin() {
    // Create hash of pin
    let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
    let pin_hash = hash(&pin);

    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticating(pin_hash),
        keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_enter_single_digit_of_withdraw_amount() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: Vec::new(),
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::One));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };

    assert_eq!(end, expected);

    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end1 = Atm::next_state(&mut start, &Action::PressKey(Key::Four));
    let expected1 = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };

    assert_eq!(end1, expected1);
}

#[test]
fn sm_3_try_to_withdraw_too_much() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One, Key::Four],
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}

#[test]
fn sm_3_withdraw_acceptable_amount() {
    let mut start = Atm {
        cash_inside: 10,
        expected_pin_hash: Auth::Authenticated,
        keystroke_register: vec![Key::One],
    };
    let end = Atm::next_state(&mut start, &Action::PressKey(Key::Enter));
    let expected = Atm {
        cash_inside: 9,
        expected_pin_hash: Auth::Waiting,
        keystroke_register: Vec::new(),
    };

    assert_eq!(end, expected);
}