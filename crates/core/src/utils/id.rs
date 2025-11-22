#[macro_export]
macro_rules! create_entity_id {
    ($name: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        pub struct $name(pub uuid::NonNilUuid);

        impl $name {
            pub fn new() -> Self {
                Self(
                    uuid::NonNilUuid::new(uuid::Uuid::now_v7())
                        .expect("UUID v7 should never be nil"),
                )
            }
        }

        impl TryFrom<uuid::Uuid> for $name {
            type Error = uuid::Error;

            fn try_from(value: uuid::Uuid) -> Result<Self, Self::Error> {
                uuid::NonNilUuid::try_from(value).map($name)
            }
        }

        impl From<$name> for uuid::Uuid {
            fn from(id: $name) -> Self {
                id.0.into()
            }
        }

        impl From<&$name> for uuid::Uuid {
            fn from(id: &$name) -> Self {
                id.0.into()
            }
        }
    };
}
