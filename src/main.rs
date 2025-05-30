use dialoguer::{Input, Select, theme::ColorfulTheme};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const MIN_VAL: i32 = 0;
const MAX_VAL: i32 = 150;

fn main() {
    let mut actions: HashMap<&str, i32> = HashMap::new();
    actions.insert("light hit", -3);
    actions.insert("medium hit", -6);
    actions.insert("hard hit", -9);
    actions.insert("hard hit", -9);
    actions.insert("draw", -15);

    actions.insert("punch", 2);
    actions.insert("bend", 7);
    actions.insert("upset", 13);
    actions.insert("shrink", 16);

    let offset = Input::with_theme(&ColorfulTheme::default())
        .validate_with(|s: &String| -> Result<(), String> {
            if s.parse::<u32>().is_err() {
                Err(format!("\"{}\" is not a valid number", s))
            } else {
                Ok(())
            }
        })
        .with_prompt("Enter the position of the arrow")
        .interact()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let actions_name: Vec<&&str> = actions.keys().collect();
    let selected_action1 = Select::with_theme(&ColorfulTheme::default())
        .items(&actions_name)
        .with_prompt("Select the first action")
        .interact()
        .unwrap();
    let selected_action2 = Select::with_theme(&ColorfulTheme::default())
        .items(&actions_name)
        .with_prompt("Select the second action")
        .interact()
        .unwrap();
    let selected_action3 = Select::with_theme(&ColorfulTheme::default())
        .items(&actions_name)
        .with_prompt("Select the third action")
        .interact()
        .unwrap();

    let selected_actions = vec![selected_action1, selected_action2, selected_action3];

    let action_offset = selected_actions.iter().fold(0, |sum, action_index| {
        sum + actions.get(actions_name[*action_index]).unwrap()
    });

    let resolve_res = resolve(&actions, offset + action_offset);
    println!("Result:");
    for a in resolve_res {
        println!("- {}", a);
    }
}

fn resolve(actions: &HashMap<&str, i32>, target: i32) -> Vec<String> {
    let mut distances: HashMap<i32, u32> = HashMap::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut came_from: HashMap<i32, i32> = HashMap::new();
    let mut queue: Vec<i32> = Vec::new();

    distances.insert(0, 0);
    queue.push(0);

    while !queue.is_empty() {
        let mut lowest_index = 0;
        let mut lowest_dist = u32::MAX;

        for (i, e) in queue.iter().enumerate() {
            let curr_dist = *distances.get(e).unwrap_or(&u32::MAX);
            if curr_dist < lowest_dist {
                lowest_index = i;
                lowest_dist = curr_dist;
            }
        }
        let current = queue[lowest_index];
        queue.swap_remove(lowest_index);
        let curr_dist = *distances.get(&current).unwrap_or(&u32::MAX);

        for (_, action_value) in actions {
            let next_val = current + action_value;
            if visited.contains(&next_val) {
                continue;
            }
            if next_val < MIN_VAL || next_val >= MAX_VAL {
                continue;
            }

            if !queue.contains(&next_val) && next_val != target {
                queue.push(next_val)
            }
            let new_dist = curr_dist + 1;
            let previous_dist = *distances.get(&next_val).unwrap_or(&u32::MAX);
            if new_dist < previous_dist {
                distances.insert(next_val, new_dist);
                came_from.insert(next_val, current);
            }
            visited.insert(current);
        }
    }

    // rebuild path
    let mut path = vec![target];
    let mut current = target;
    while let Some(parent) = came_from.get(&current) {
        current = *parent;
        path.push(current);
    }
    path.reverse();

    let mut path2: Vec<i32> = vec![];
    for (a, b) in path.iter().tuple_windows() {
        path2.push(b - a);
    }
    path2.sort();
    path2.reverse();

    let mut output = Vec::new();
    for action_value in path2 {
        let action = actions
            .iter()
            .find(|(_, val)| **val == action_value)
            .expect("failed to find action");
        output.push(action.0.to_string());
    }

    return output;
}
