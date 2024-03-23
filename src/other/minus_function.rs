pub fn sub_five(num: u32) -> u32{
    num - 5
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sub_five_test() {
        assert_eq!(sub_five(5), 0);
    }
}