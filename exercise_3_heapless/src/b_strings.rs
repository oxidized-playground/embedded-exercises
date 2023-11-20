use heapless::{String, Vec};

pub fn give_me_a_heart() -> String<4> {
    // To make a heart you need to use these bytes
    let mut sparkle_heart = Vec::<u8, 4>::new();
    let _ = sparkle_heart.extend_from_slice(&[240, 159, 146, 150]);

    todo!()
}

pub fn give_me_a_pizza_heart() -> String<12> {
    // Combine this pizza with some love!
    let pizza: &str = "🍕️";
    let heart = give_me_a_heart();

    // We have an empty string here, can you append both together? Hint: You can use core::fmt::Write for this
    let mut pizza_heart: String<12> = String::new();
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::b_strings::{give_me_a_heart, give_me_a_pizza_heart};

    #[test]
    fn test_give_me_a_heart() {
        let sparkle_heart = give_me_a_heart();
        assert_eq!("💖", sparkle_heart);
    }

    #[test]
    fn test_give_me_a_pizza_heart() {
        let sparkling_pizza_heart = give_me_a_pizza_heart();
        assert_eq!("🍕️💖", sparkling_pizza_heart);
    }
}
