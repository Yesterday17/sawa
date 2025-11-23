use aide::{
    axum::{
        ApiRouter, AxumOperationHandler, IntoApiResponse,
        routing::{ApiMethodRouter, get},
    },
    openapi::OpenApi,
};
use axum::{Extension, Json, response::Html};
use std::sync::Arc;

#[must_use]
pub struct Scalar {
    html: &'static str,
}

impl Scalar {
    pub fn new(title: Option<&str>, spec_url: &str, api_url: Option<&str>) -> Self {
        let title = match title {
            Some(title) => title.to_string(),
            None => "API Documentation".to_string(),
        };

        let mut configuration = serde_json::json!({
            "theme": "default",
            "spec": {
                "url": spec_url,
            },
        });

        if let Some(api_url) = &api_url {
            configuration["servers"] = serde_json::json!([
                {
                    "description": "API Server",
                    "url": api_url
                }
            ]);
        }

        let html = format!(
            r#"
            <!DOCTYPE html>
            <html>
              <head>
                <title>{title}</title>
                <meta charset="utf-8" />
                <meta
                  name="viewport"
                  content="width=device-width, initial-scale=1" />
                <style>
                  body {{
                    margin: 0;
                 }}
                </style>
              </head>
              <body>
                <script
                  id="api-reference"></script>
                <script>
                  var apiReference = document.getElementById('api-reference')
                  apiReference.dataset.configuration = '{configuration}'
                </script>
                <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference/dist/browser/standalone.min.js"></script>
              </body>
            </html>
            "#,
            title = title,
            configuration = serde_json::to_string(&configuration).unwrap()
        );

        Self {
            html: Box::leak(html.into_boxed_str()),
        }
    }
}

impl Scalar {
    pub fn axum_route<S>(&self) -> ApiMethodRouter<S>
    where
        S: Clone + Send + Sync + 'static,
    {
        get(self.axum_handler())
    }

    #[must_use]
    pub fn axum_handler<S>(&self) -> impl AxumOperationHandler<(), Html<&'static str>, ((),), S> {
        let html = self.html;
        move || async move { Html(html) }
    }
}

pub fn docs_routes(api: OpenApi) -> ApiRouter {
    ApiRouter::new()
        .route(
            "/scalar",
            Scalar::new(
                Some("Sawa API Documentation"),
                "/docs/private/api.json",
                None,
            )
            .axum_route(),
        )
        .api_route("/docs/private/api.json", get(serve_docs))
        .layer(Extension(Arc::new(api)))
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api)
}
