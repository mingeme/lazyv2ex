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
    LineUp(u32),
    LineDown(u32),
    OpenBrowser(String),
    Quit,
}
