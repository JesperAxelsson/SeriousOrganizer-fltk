#[derive(Debug,  Clone)]
pub enum Message {
    // Label Table Events
    LabelTableInvalidated,
    
    // Entry Table Events
    EntryTableInvalidated,
    EntryTableSortCol(i32),
    EntryChanged(usize),
    EntryShowContextMenu(Vec<u32>),

    // File table Events
    FileTableInvalidated,
    FileTableSortCol(i32),
    FileTableChanged(usize),
    FileShowContextMenu(Vec<u32>),
    FileTableOpen,
}
