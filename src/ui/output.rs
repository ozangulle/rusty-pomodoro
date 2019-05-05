use crate::uimessages::UIMessages;

pub trait Output: Sync + Send {
    fn display(&self, ui_message: UIMessages) -> ();
}
