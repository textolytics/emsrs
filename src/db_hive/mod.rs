use faststr::FastStr;
use storage::StorageDescriptor;

pub struct ThriftHiveMetastoreCreateTableArgsSend {
    pub tbl: Table,
}

pub struct Table {

    pub table_name: Option<FastStr>,
    pub db_name: Option<FastStr>,
    pub owner: Option<FastStr>,
    pub create_time: Option<i32>,
    pub last_access_time: Option<i32>,
    pub retention: Option<i32>,
    pub sd: Option<StorageDescriptor>,
    pub partition_keys: Option<Vec<FieldSchema>>,
    pub parameters: Option<AHashMap<FastStr, FastStr>>,
    pub view_original_text: Option<FastStr>,
    pub view_expanded_text: Option<FastStr>,
    pub table_type: Option<FastStr>,
    pub privileges: Option<PrincipalPrivilegeSet>,
    pub temporary: Option<bool>,
    pub rewrite_enabled: Option<bool>,
    pub cat_name: Option<FastStr>,
}