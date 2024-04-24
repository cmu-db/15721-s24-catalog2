use crate::server::routes::common::*;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
// #[derive(serde::Deserialize)]
pub struct CreateTableRequest {
  pub name: String,
  // location: Option<String>,
  // schema: Schema,
  // #[serde(rename = "partition-spec")]
  // partition_spec: PartitionSpec,
  // #[serde(rename = "write-order")]
  // write_order: SortOrder,
  // #[serde(rename = "stage-create")]
  // stage_create: bool,
  // properties: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[allow(dead_code)]
pub struct RegisterTableRequest {
  name: String,
  #[serde(rename = "metadata-location")]
  metadata_location: String, // TODO: need to modify to table schema
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CommitTableRequest {
  // pub identifier: TableIdentifier,
  pub requirements: Vec<TableRequirement>,
  pub updates: Vec<TableUpdate>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct TableRequirement {
  #[serde(flatten)]
  pub requirement: RequirementType,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum RequirementType {
  // AssertCreate(AssertCreate),
  // AssertTableUUID(AssertTableUUID),
  // AssertRefSnapshotId(AssertRefSnapshotId),
  // AssertLastAssignedFieldId(AssertLastAssignedFieldId),
  // AssertCurrentSchemaId(AssertCurrentSchemaId),
  // AssertLastAssignedPartitionId(AssertLastAssignedPartitionId),
  // AssertDefaultSpecId(AssertDefaultSpecId),
  // AssertDefaultSortOrderId(AssertDefaultSortOrderId),
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(untagged)]
pub enum TableUpdate {
  // AssignUUIDUpdate(AssignUUIDUpdate),
  // UpgradeFormatVersionUpdate(UpgradeFormatVersionUpdate),
  // AddSchemaUpdate(AddSchemaUpdate),
  // SetCurrentSchemaUpdate(SetCurrentSchemaUpdate),
  // AddPartitionSpecUpdate(AddPartitionSpecUpdate),
  // SetDefaultSpecUpdate(SetDefaultSpecUpdate),
  // AddSortOrderUpdate(AddSortOrderUpdate),
  // SetDefaultSortOrderUpdate(SetDefaultSortOrderUpdate),
  // AddSnapshotUpdate(AddSnapshotUpdate),
  // SetSnapshotRefUpdate(SetSnapshotRefUpdate),
  // RemoveSnapshotsUpdate(RemoveSnapshotsUpdate),
  // RemoveSnapshotRefUpdate(RemoveSnapshotRefUpdate),
  // SetLocationUpdate(SetLocationUpdate),
  // SetPropertiesUpdate(SetPropertiesUpdate),
  // RemovePropertiesUpdate(RemovePropertiesUpdate),
}

#[derive(FromForm)]
#[allow(dead_code)]
pub struct PurgeRequested {
  purge_requested: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RenameTableRequest {
  pub source: TableIdentifier,
  pub destination: TableIdentifier,
}
