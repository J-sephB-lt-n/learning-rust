
```
└── src
    ├── cloud
    │   ├── aws
    │   │   ├── mod.rs
    │   │   └── s3.rs
    │   ├── gcp
    │   │   ├── mod.rs
    │   │   └── cloud_storage.rs
    │   └── mod.rs
    └── main.rs
```

```rust
// cloud/mod.rs
pub mod aws;
pub mod gcp;

// cloud/aws/mod.rs
pub mod s3;

// cloud/gcp/mod.rs
pub mod cloud_storage;

// main.rs
mod cloud;
use cloud::gcp::cloud_storage;
fn main() {
    cloud::aws::s3::download_file();
    cloud::aws::s3::upload_file();

    cloud_storage::download_file();
    cloud_storage::upload_file();
}
```

```bash
$ cargo run
called src::cloud::aws::s3::download_file
called src::cloud::aws::s3::upload_file
called src::cloud::gcp::cloud_storage::download_file
called src::cloud::gcp::cloud_storage::upload_file
```

