// <https://github.com/opencontainers/distribution-spec/blob/v1.1.0/spec.md>
#[rustfmt::skip]
pub const ROUTES: [&str; 7] = [
    "/v2",
    "/v2/{name:*}/blobs/{digest}",
    "/v2/{name:*}/blobs/uploads",
    "/v2/{name:*}/blobs/uploads/{reference}",
    "/v2/{name:*}/manifests/{reference}",
    "/v2/{name:*}/referrers/{digest}",
    "/v2/{name:*}/tags/list",
];
