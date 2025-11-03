impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut i = 0;

        while i < flowerbed.len() && n > 0 {
            // If the current plot is planted, the next one can't be planted
            if flowerbed[i] == 1 {
                i += 2;
                continue;
            }

            // If the right plot is planted, the next two can't be used
            if !(i + 1 == flowerbed.len() || flowerbed[i + 1] == 0) {
                i += 3;
                continue;
            }

            // If the left plant is planted, we could plant in the next plot
            if !(i == 0 || flowerbed[i - 1] == 0){
                i += 1;
                continue
            }

            // We can plant here, so we decrease n and skip the next plot,
            // as the current plot is now occupied
            n -= 1;
            i += 2;
        }
        
        n == 0
    }
}  
