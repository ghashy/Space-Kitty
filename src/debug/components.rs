// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Hash, Clone)]
pub struct AddValueToDebugEvent(pub String, pub String);

impl PartialEq for AddValueToDebugEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl Eq for AddValueToDebugEvent {}
