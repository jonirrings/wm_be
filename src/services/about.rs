//! Templates for "about" static pages.
use crate::web::api::v1::routes::API_VERSION_URL_PREFIX;

#[must_use]
pub fn index_page() -> String {
    page()
}

#[must_use]
pub fn page() -> String {
    format!(
        r#"
    <html>
        <head>
            <title>About</title>
        </head>
        <body style="margin-left: auto;margin-right: auto;max-width: 30em;">
            <h1>Warehouse Management</h1>

            <h2>About</h2>

            <p>Hi! This is a running Warehouse Management.</p>
        </body>
        <footer style="padding: 1.25em 0;border-top: dotted 1px;">
            <a href="/{API_VERSION_URL_PREFIX}/about/license">license</a>
        </footer>
    </html>
"#
    )
}

#[must_use]
pub fn license_page() -> String {
    format!(
        r#"
    <html>
        <head>
            <title>Licensing</title>
        </head>
        <body style="margin-left: auto;margin-right: auto;max-width: 30em;">
            <h1>Warehouse Management</h1>

            <h2>Licensing</h2>

            <h3>Multiple Licenses</h3>

        </body>
        <footer style="padding: 1.25em 0;border-top: dotted 1px;">
            <a href="/{API_VERSION_URL_PREFIX}/about">about</a>
        </footer>
    </html>
"#
    )
}
