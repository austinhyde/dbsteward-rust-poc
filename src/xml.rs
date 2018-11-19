macro_rules! elem {
  ($struct_name:ident as $parent_name:expr; $($field_name:ident $type:ty as $elem_name:expr),*) => {
    #[derive(Debug, Deserialize)]
    #[serde(rename = $parent_name)]
    pub struct $struct_name {
      $(
        #[serde(rename = $elem_name)]
        pub $field_name: $type,
      )*
    }
  };
  ($struct_name:ident; $($field_name:ident $type:ty as $elem_name:expr),*) => {
    #[derive(Debug, Deserialize)]
    pub struct $struct_name {
      $(
        #[serde(rename = $elem_name)]
        pub $field_name: $type,
      )*
    }
  };
}

elem!{DBSteward as "dbsteward";
  database Database as "database"
}

elem!{Database;
  schemas Vec<Schema> as "schema"
}

elem!{Schema;
  name String as "name",
  owner String as "owner",
  tables Vec<Table> as "table"
}

elem!{Table;
  name String as "name",
  owner String as "owner",
  columns Vec<Column> as "column",
  indexes Vec<Index> as "index"
}

elem!{Column;
  name String as "name",
  data_type String as "type"
}

elem!{Index;
  name String as "name",
  using String as "using",
  dimensions Vec<IndexDimension> as "indexDimension"
}

elem!{IndexDimension;
  value String as "$value"
}
