pub use path_dep_of_path_dep::is_sum;

pub fn add(x: usize, y: usize) -> usize {
    x + y
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
