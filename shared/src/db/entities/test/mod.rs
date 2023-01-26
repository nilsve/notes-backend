use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct TestEntityId(String);

#[derive(Serialize, Deserialize)]
pub struct TestEntity {
    id: TestEntityId,
}
