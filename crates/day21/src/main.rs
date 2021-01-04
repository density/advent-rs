use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn read_data() -> (
    HashMap<&'static str, usize>,
    HashMap<&'static str, HashSet<&'static str>>,
) {
    let mut allergen_to_ingredients = HashMap::new();

    let mut ingredient_counts = HashMap::new();

    for line in INPUT.lines() {
        let mut split = line.split(" (contains ");

        let ingredients: HashSet<&'static str> =
            split.next().unwrap().split_ascii_whitespace().collect();

        for ingredient in &ingredients {
            *ingredient_counts.entry(*ingredient).or_insert(0) += 1
        }

        let allergen_str = split.next().unwrap();

        let allergens: HashSet<&'static str> =
            allergen_str[..allergen_str.len() - 1].split(", ").collect();

        for allergen in allergens {
            match allergen_to_ingredients.get_mut(allergen) {
                None => {
                    allergen_to_ingredients.insert(allergen, ingredients.clone());
                }
                Some(ingredients_so_far) => {
                    ingredients_so_far.retain(|ing| ingredients.contains(ing));
                }
            }
        }
    }

    (ingredient_counts, allergen_to_ingredients)
}

fn part1() -> usize {
    let (all_ingredients, allergen_to_ingredients) = read_data();

    all_ingredients
        .iter()
        .filter_map(|(ingredient, count)| {
            if allergen_to_ingredients
                .values()
                .all(|ingredients| !ingredients.contains(ingredient))
            {
                Some(count)
            } else {
                None
            }
        })
        .sum()
}

fn part2() -> String {
    let (_, mut allergen_to_ingredients) = read_data();

    let mut allergen_to_ingredient = BTreeMap::new();

    while !allergen_to_ingredients.is_empty() {
        let single_ingredient_allergen = *allergen_to_ingredients
            .keys()
            .find(|allergen| allergen_to_ingredients[*allergen].len() == 1)
            .unwrap();

        let (allergen, ingredient) = allergen_to_ingredients
            .remove_entry(single_ingredient_allergen)
            .unwrap();

        let found_ingredident = ingredient.into_iter().next().unwrap();

        for ingredient_list in allergen_to_ingredients.values_mut() {
            ingredient_list.remove(&found_ingredident);
        }

        allergen_to_ingredient.insert(allergen, found_ingredident);
    }

    let final_ingredients: Vec<String> = allergen_to_ingredient
        .values()
        .map(|&v| v.to_owned())
        .collect();
    final_ingredients.join(",")
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", (Instant::now() - start).as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2317);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "kbdgs,sqvv,slkfgq,vgnj,brdd,tpd,csfmb,lrnz");
    }
}
