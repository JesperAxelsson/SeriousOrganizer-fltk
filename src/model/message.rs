#[derive(Debug, Copy, Clone)]
pub enum Message {
    LabelTableInvalidated,
    EntryTableInvalidated,
    EntryTableSortCol(i32),
    EntryChanged(usize),
}
