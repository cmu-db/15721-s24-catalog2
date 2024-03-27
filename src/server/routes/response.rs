use rocket::serde::Serialize;


// #[get("/namespaces/<namespace>/tables")] --> 200: ListTablesResponse
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ListTablesResponse {
    pub identifiers: Vec<TableIdentifier>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TableIdentifier {
    pub namespace: Namespace,
    pub name: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Namespace(
    pub Vec<String>
);


// #[post("/namespaces/<namespace>/tables")] --> 200: CreateTableResponse
// #[post("/namespaces/<namespace>/register")] --> 200: LoadTableResponse
// #[get("/namespaces/<namespace>/tables/<table>")] --> 200: LoadTableResponse
// LoadTableResult
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoadTableResult {
    // pub metadata_location: Option<String>,
    pub metadata: TableMetadata,
    // pub config: HashMap<String, String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TableMetadata {
    pub format_version: i32,
    pub table_uuid: String,
    // pub location: Option<String>,
    // pub last_updated_ms: Option<i64>,
    // pub properties: HashMap<String, String>,
    // pub schemas: Vec<Schema>,
    // pub current_schema_id: Option<i32>,
    // pub last_column_id: Option<i32>,
    // pub partition_specs: Vec<PartitionSpec>,
    // pub default_spec_id: Option<i32>,
    // pub last_partition_id: Option<i32>,
    // pub sort_orders: Vec<SortOrder>,
    // pub default_sort_order_id: Option<i32>,
    // pub snapshots: Vec<Snapshot>,
    // pub refs: SnapshotReferences,
    // pub current_snapshot_id: Option<i64>,
    // pub last_sequence_number: Option<i64>,
    // pub snapshot_log: SnapshotLog,
    // pub metadata_log: MetadataLog,
}


// 400: BadRequestErrorResponse
// 404: IcebergErrorResponse
// 409: TableAlreadyExistsError
// 503: ServiceUnavailableResponse
// 5XX: ServerErrorResponse
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct IcebergErrorResponse {
    pub error: ErrorModel,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorModel {
    pub message: String,
    pub r#type: String, // Using r#type to avoid conflict with the type keyword
    pub code: i32,
    // pub stack: Option<Vec<String>>,
}

