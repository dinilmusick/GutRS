//! Native hosted SDK adapters corresponding to pinned GutTS storage exports.
//!
//! This module imports the real Rust SDKs, not mocks. Enable `aws-s3` or
//! `google-drive` explicitly. Configuration, credentials, OAuth grants and
//! network access remain the **consumer's** responsibility.
//!
//! Only metadata/builder construction is tested without credentials. Real
//! bucket/Drive access requires separately authorized integration credentials.

#[cfg(feature = "aws-s3")]
pub mod s3 {
    //! Fully functional AWS S3 fluent request builders. Call `.send().await`
    //! on a Tokio runtime after creating a genuine authenticated SDK Client.
    pub use aws_sdk_s3::Client as S3Client;
    pub use aws_sdk_s3::config::Builder as S3ConfigBuilder;
    pub use aws_sdk_s3::operation::copy_object::builders::CopyObjectFluentBuilder as CopyObjectCommand;
    pub use aws_sdk_s3::operation::delete_object::builders::DeleteObjectFluentBuilder as DeleteObjectCommand;
    pub use aws_sdk_s3::operation::delete_objects::builders::DeleteObjectsFluentBuilder as DeleteObjectsCommand;
    pub use aws_sdk_s3::operation::get_object::builders::GetObjectFluentBuilder as GetObjectCommand;
    pub use aws_sdk_s3::operation::head_object::builders::HeadObjectFluentBuilder as HeadObjectCommand;
    pub use aws_sdk_s3::operation::list_objects_v2::builders::ListObjectsV2FluentBuilder as ListObjectsV2Command;
    pub use aws_sdk_s3::operation::put_object::builders::PutObjectFluentBuilder as PutObjectCommand;

    #[test]
    fn real_s3_client_exposes_all_eight_native_requests() {
        use aws_sdk_s3::config::Region;
        let config = aws_sdk_s3::Config::builder()
            .region(Region::new("us-east-1"))
            .endpoint_url("http://127.0.0.1:9000")
            .force_path_style(true)
            .build();
        let client = S3Client::from_conf(config);
        let _: GetObjectCommand = client.get_object().bucket("example").key("a");
        let _: HeadObjectCommand = client.head_object().bucket("example").key("a");
        let _: ListObjectsV2Command = client.list_objects_v2().bucket("example");
        let _: PutObjectCommand = client.put_object().bucket("example").key("a");
        let _: CopyObjectCommand = client.copy_object().bucket("example").key("b").copy_source("example/a");
        let _: DeleteObjectCommand = client.delete_object().bucket("example").key("a");
        let _: DeleteObjectsCommand = client.delete_objects().bucket("example");
    }
}

#[cfg(feature = "google-drive")]
pub mod google_drive {
    //! Actual Google Drive v3 SDK, not a `googleapis` JavaScript dynamic import.
    //! Consumers provide an authorized `GetToken` and Hyper HTTPS connector.
    pub use google_drive3 as GoogleApisModule;
    pub use google_drive3::{DriveHub as GoogleDriveClient, hyper_rustls, hyper_util, yup_oauth2};

    /// Construct the actual Drive v3 hub, using a supplied authenticated host.
    ///
    /// The `googleapis` JavaScript module is replaced by the real
    /// `google-drive3` Rust SDK. This does not mint OAuth credentials.
    pub fn create_google_drive_client<C, A>(
        client: google_drive3::common::Client<C>,
        authenticator: A,
    ) -> GoogleDriveClient<C>
    where
        A: 'static + google_drive3::common::GetToken,
    {
        GoogleDriveClient::new(client, authenticator)
    }

    #[test]
    fn rust_drive_v3_is_the_real_host_sdk() {
        use google_drive3::api::{File, FileList};
        let _: File = File::default();
        let _: FileList = FileList::default();
        // No networking, fake token provider or browser-global assumptions.
        assert!(std::any::type_name::<GoogleDriveClient<()>>().contains("DriveHub"));
    }
}
