pub mod mementos {
    table! {
        mementos.mementos (id) {
            id -> Uuid,
            prompt -> Text,
            details -> Text,
        }
    }
}
