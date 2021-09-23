#[derive(Debug,  Clone)]
pub enum Message {
    LabelTableInvalidated,
    EntryTableInvalidated,
    EntryTableSortCol(i32),
    EntryChanged(usize),
    EntryShowContextMenu(Vec<u32>),
}
