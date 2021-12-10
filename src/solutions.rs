pub trait Solution<T> {
    fn parse_file() -> T;
    fn part_a(data: &T);
    fn part_b(data: &T);
}
