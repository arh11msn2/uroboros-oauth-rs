pub mod responses {
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct OkResponse {
        pub ok: bool,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    pub struct StringIdResponse {
        pub id: String,
    }
}
