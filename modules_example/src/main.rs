mod cloud;
use cloud::gcp::cloud_storage;
fn main() {
    cloud::aws::s3::download_file();
    cloud::aws::s3::upload_file();

    cloud_storage::download_file();
    cloud_storage::upload_file();
}
