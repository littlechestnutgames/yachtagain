mod shared;
use crate::shared::Score;
use crate::shared::ScoringRule;
use crate::shared::Die;
use crate::shared::ApplyScore;
use crate::shared::SumType;
fn main() {
    let ones_rule = Score {
        name: "Ones".to_string(),
        scored: 0,
        scoring_rule: ScoringRule::Face(1, SumType::MatchingFaces)
    };
    let twos_rule = Score {
        name: "Twos".to_string(),
        scored: 6,
        scoring_rule: ScoringRule::Face(2, SumType::MatchingFaces)
    };
    let threes_rule = Score {
        name: "Threes".to_string(),
        scored: 9,
        scoring_rule: ScoringRule::Face(3, SumType::All)
    };
    let fours_rule = Score {
        name: "Fours".to_string(),
        scored: 12,
        scoring_rule: ScoringRule::Face(4, SumType::MatchingFaces)
    };
    let fives_rule = Score {
        name: "Fives".to_string(),
        scored: 15,
        scoring_rule: ScoringRule::Face(5, SumType::MatchingFaces)
    };
    let sixes_rule = Score {
        name: "Sixes".to_string(),
        scored:    18,
        scoring_rule: ScoringRule::Face(6, SumType::MatchingFaces)
    };
    let three_of_a_kind = Score {
        name: "Three of a kind".to_string(),
        scored: 0,
        scoring_rule: ScoringRule::Groups(vec![3], SumType::All)
    };
    let full_house: Score = Score {
        name: "Full House".to_string(),
        scored: 0,
        scoring_rule: ScoringRule::Groups(vec![3,2], SumType::ExactAmount(25))
    };
    let bonus: Score = Score {
        name: "Upper Bonus".to_string(),
        scored: 0,
        scoring_rule: ScoringRule::ScoreDependent(
            vec![ones_rule.clone(), twos_rule.clone(), threes_rule.clone(), fours_rule.clone(), fives_rule.clone(), sixes_rule.clone()],
            63,
            SumType::ExactAmount(35)
        )
    };
    
    let rules = vec![
        ones_rule,
        twos_rule,
        threes_rule,
        fours_rule,
        fives_rule,
        sixes_rule,
        bonus,
        three_of_a_kind,
        full_house
    ];
    
    let mut dice = Vec::new();
    for i in 1..6 {
        let mut d = Die {
            name: "d6".to_string(),
            faces: vec![1,2,3,4,5,6],
            rolled_value: 3,
            held: false,
            removable: false
        };
        // d.roll();
        dice.push(d);
    }
    
    for rule in rules.iter() {
        println!("Sum of {} -> {}", rule.name, rule.apply(dice.clone()));
    }
}