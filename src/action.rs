#[derive(PartialEq, Debug)]
pub enum Action {
    PreviousRow,
    NextRow,
    Top,
    Bottom,
    Enter,
    Reload,
    FetchTopics,
    FetchTopicDetail(String),
    GoHome,
    LineUp(u16),
    LineDown(u16),
    OpenBrowser(String),
    Quit,
}
