use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

// We need two different maps:
// 1. Store the cusine and rating for a given food
// 2. Store all foods in order for a given cusine
struct FoodRatings {
    foods: HashMap<String, (i32, String)>,
    cuisines: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        Self {
            foods: (0..foods.len())
                .map(|i| (foods[i].clone(), (ratings[i], cuisines[i].clone())))
                .collect(),
            cuisines: (0..cuisines.len()).fold(HashMap::new(), |mut acc, i| {
                acc.entry(cuisines[i].clone())
                    .or_insert(BinaryHeap::new())
                    .push((ratings[i], Reverse(foods[i].clone())));

                acc
            }),
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let (rating, cuisine) = self.foods.get_mut(&food).unwrap();

        // Change the current rating for the food map
        *rating = new_rating;

        // Add the new rating to the corresponding heap for the cuisine
        self.cuisines
            .entry(cuisine.to_string())
            .and_modify(|heap| heap.push((new_rating, Reverse(food))));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.cuisines.get_mut(&cuisine).unwrap();

        // We lazily remove old entries
        loop {
            // We check if the current top rating for the given cuisine
            // is still the current rating
            let (rating, Reverse(food)) = heap.peek().unwrap();

            // Compare rating with actual rating
            if *rating == self.foods.get(food).unwrap().0 {
                return food.to_string();
            }

            // Remove the entry, as the entry is not the latest.
            heap.pop();
        }
    }
}
