#[derive(PartialEq, Debug)]
pub enum Action {
    PreviousRow,
    NextRow,
    Top,
    Bottom,
    Enter,
    Reload,
    FetchTopics,
    Quit,
}
