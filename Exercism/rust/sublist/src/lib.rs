#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (a, b) if a > b => if _first_list.windows(b).any(|item| item == _second_list) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
        (a, b) if b > a => if _second_list.windows(a).any(|item| item == _first_list) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
        (_, _) => if _first_list == _second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    }
}
