# DBSteward Rust Proof-of-Concept

I wanted to see what doing a re-write in Rust would look like, so here's the beginnings of it.

Notably

* CLI flags parsed by [clap](https://github.com/clap-rs/clap), configured in `cli.yml`
* XML parsed by [serde](https://github.com/serde-rs/serde)/[serde-xml-rs](https://github.com/RReverser/serde-xml-rs), configured in `xml.rs`
* Using some macros to alleviate deserialization struct boilerplate

So, for example,

```rust
elem!{Table;
  name String as "name",
  owner String as "owner",
  columns Vec<Column> as "column",
  indexes Vec<Index> as "index"
}
```

expands to

```rust
#[derive(Debug, Deserialize)]
pub struct Table {
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "owner")]
  owner: String,
  #[serde(rename = "column")]
  columns: Vec<Column>,
  #[serde(rename = "index")]
  indexes: Vec<Index>,
}
```

and will parse

```xml
<table name="user" owner="$OWNER" primaryKey="id" description="users table">
  <column name="id" type="uuid"/>
  <column name="username" type="varchar(40)"/>
  <column name="password" type="text"/>
  <column name="registered_at" type="timestamptz"/>
  <index name="username_idx" using="btree">
    <indexDimension>username</indexDimension>
  </index>
</table>
```

into

```
Table {
  name: "user",
  owner: "$OWNER",
  columns: [
    Column { name: "id", data_type: "uuid" },
    Column { name: "username", data_type: "varchar(40)" },
    Column { name: "password", data_type: "text" },
    Column { name: "registered_at", data_type: "timestamptz" }
  ],
  indexes: [
    Index {
      name: "username_idx", using: "btree",
      dimensions: [
        IndexDimension { value: "username" }
      ]
    }
  ]
}
```
