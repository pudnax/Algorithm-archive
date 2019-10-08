extern crate rand;

use crate::rand::prelude::SliceRandom;

struct Person {
    /*
      this is a poor person's std::optional,
      but since we're attempting to be compileable on C++14,
      we won't worry too much about it.
    */
    finished: bool,
    preference: usize,
    preference_list: Vec<usize>,
}

/*
  this function generates a list of people with size `number_of_partners`.

  each person's `preference_list` will be a randomly sorted list of
  the numbers in the range [0, number_of_partners),
  with no duplicates.
*/
fn make_person_list(number_of_partners: usize) -> Vec<Person> {
    let mut rng = rand::thread_rng();

    let mut random_pref_list = |number_of_partners| {
        let mut temp: Vec<_> = (0..number_of_partners).collect();
        temp.shuffle(&mut rng);
        temp
    };

    (0..number_of_partners)
        .map(|_x| Person {
            finished: false,
            preference: 0,
            preference_list: random_pref_list(number_of_partners),
        })
        .collect::<Vec<_>>()
}

fn stable_match(leads: &mut Vec<Person>, follows: &mut Vec<Person>) {
    let number_of_partners = leads.len();

    for proposal_index in 0..number_of_partners {
        let mut proposals: Vec<Vec<usize>> = (0..number_of_partners).map(|_x| Vec::new()).collect();

        for i in 0..number_of_partners {
            if !leads[i].finished {
                let pref = leads[i].preference_list[proposal_index];
                proposals[pref].push(i);
            }
        }

        for i in 0..number_of_partners {
            for (_, pref) in (0..follows[i].preference_list.len()).enumerate() {
                for proposal in &proposals[i] {
                    if pref == *proposal && !follows[i].finished {
                        follows[i].preference = pref;
                        follows[i].finished = true;

                        leads[pref].preference = i;
                        leads[pref].finished = true;
                    }
                }
            }
        }
    }
}

fn main() {
    let number_of_partners = 500;

    let mut leads = make_person_list(number_of_partners);
    let mut follows = make_person_list(number_of_partners);

    stable_match(&mut leads, &mut follows);

    for (i, lead) in leads.iter().enumerate().take(number_of_partners) {
        println!(
            "the partnership of lead {} and follow {} shall commence forthwith!",
            i, lead.preference
        );
    }
}

