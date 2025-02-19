#[derive(PartialEq, Debug)]
pub enum Action {
    PreviousRow,
    NextRow,
    Top,
    Bottom,
    Enter,
    FetchTopics,
    Quit,
    Noop,
}
