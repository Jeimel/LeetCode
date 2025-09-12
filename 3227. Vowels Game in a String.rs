impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        // There exist two difference cases:
        //  1. We have an odd number of vowels
        //  2. We have an even number of vowels
        //
        // In the first case, Alice can just take all vowels and wins.
        //
        // In the second case, Alice can take all vowels, except one.
        // Then, for Bob there exists no non-empty substring with even 
        // number of vowels, which results in Alice winning again.
        //
        // However, we have one edge case, which is zero vowels. Zero is even, 
        // but Alice cannot remove any substring on her first turn. This is the
        // only case, where Bob wins. Therefore, we just have to check if there
        // exists any vowel in the string, which instanly results in a win for Alice
        s.bytes().any(|b| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u'))
    }
}
