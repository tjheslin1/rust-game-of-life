pub trait Ruleset {
    fn next(&self) -> Self;
}
