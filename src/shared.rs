use std::collections::HashMap;

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    TitleScreen,
    MainMenu,
}
pub struct YachtAgainStageEvent {
    pub message: String,
}

pub trait ApplyScore {
    fn apply(&self, dice: Vec<Die>) -> i32;
}

#[derive(Clone)]
pub enum ScoringRule {
    Face(i32, SumType),
    Groups(Vec<i32>, SumType),
    ScoreDependent(Vec<Score>, i32, SumType)
}

#[derive(Clone)]
pub enum SumType {
    // This uses the Score's ExactAmount field to determine the score payout.
    ExactAmount(i32),
    // This uses the sum of the matching values in the roll to detemine the score payout.
    MatchingFaces,
    // This sums all the dice on the roll, regardless if they match or not.
    All
}

#[derive(Clone)]
pub struct Score {
    pub name: String,
    pub scored: i32,
    pub scoring_rule: ScoringRule
}

impl ApplyScore for Score {
    fn apply(&self, dice: Vec<Die>) -> i32 {
        match &self.scoring_rule {
            ScoringRule::Face(i, sum_type) => {
                match sum_type {
                    SumType::MatchingFaces => dice.iter()
                                                    .map(|x|x.rolled_value)
                                                    .filter(|x|*x == *i)
                                                    .sum(),
                    SumType::ExactAmount(amount) => if dice.iter().any(|x| x.rolled_value == *i) { *amount } else { 0 },
                    SumType::All => if dice.iter().any(|x| x.rolled_value == *i) { dice.iter().map(|x| x.rolled_value).sum::<i32>() } else { 0 }
                }
            },
            ScoringRule::Groups(numbers, sum_type) => {
                let cloned_dice = dice.clone();
                // Get the frequency of each number.
                let mut m: HashMap<i32, i32> = HashMap::new();
                for d in dice.iter() {
                    let x = d.rolled_value;
                    *m.entry(x).or_default() += 1;
                }
                let mut found: Vec<i32> = vec![];
                let mut values_found: Vec<i32> = vec![];
                for x in m.into_iter() {
                    let nums = numbers.clone();
                    let x_cloned = x.clone();
                    for n in nums {
                        if n <= x.0 {
                            let n2 = n.clone();
                            found.push(n);
                            values_found.push(x_cloned.0 * x_cloned.1);
                        }
                    }
                }

                if found.len() == numbers.len() {
                    match sum_type {
                        SumType::ExactAmount(amount) => amount,
                        SumType::MatchingFaces => &values_found.iter().sum::<i32>(),
                        SumType::All => &cloned_dice.iter().map(|d| d.rolled_value).sum::<i32>(),
                    };
                }

                return 0;
            },
            ScoringRule::ScoreDependent(scores, threshold, sum_type) => {
                if scores.iter().map(|s| s.scored).sum::<i32>() >= *threshold {
                    match sum_type {
                        SumType::ExactAmount(amount) => *amount,
                        SumType::MatchingFaces => panic!("ScoreDependent rules can only be SumType::ExactAmount."),
                        SumType::All => panic!("ScoreDependent rules can only be SumType::ExactAmount.")
                    };
                }

                0
            }
        }
    }
}

#[derive(Clone)]
pub struct Die {
    pub name: String,
    pub faces: Vec<i32>,
    pub rolled_value: i32,
    pub held: bool,
    pub removable: bool
}

impl Die {
    pub fn roll(&mut self) {
        self.rolled_value = *self.faces.choose(&mut rand::thread_rng()).unwrap();
    }
}

struct PlayerConfig {
    turn_order: i32,
    name: String,
    score: Vec<Score>
}

struct ComputerConfig {
    turn_order: i32,
    score: Vec<Score>,
    strategy: ComputerStrategy
}

// A scoring group allows us to group scores by like sections so they can be
// prioritized by computing strategies or utilized by bonus scoring
// mechanisms.
struct ScoringGroup {
    // Name we can display on screen.
    name: String,
    scores: Vec<Score>
}

// ComputerStrategies are the ways the computer decides to hold dice and take
// scores. 
enum ComputerStrategy {
    HighestNonZero,
    PrioritizeGroup(ScoringGroup),
    PerfectPlay
}

// fn hold_dice(scoring_groups: &mut Vec<ScoringGroup>, dice: &mut Vec<Die>, strategy: ComputerStrategy) {
//     match strategy {
//         ComputerStrategy::HighestNonZero => for group in scoring_groups.iter_mut() {
//             for score in group.scores.iter_mut() {
//                 return 0;
//             }
//         },
//         ComputerStrategy::PrioritizeGroup(a) => 0,
//         ComputerStrategy::PerfectPlay => 0
//     };
// }

// fn select_score(scoring_group: &mut ScoringGroup, dice: &mut Vec<Die>, strategy: ComputerStrategy) {

// }

enum Player {
    Human(PlayerConfig),
    Computer(ComputerConfig)
}
