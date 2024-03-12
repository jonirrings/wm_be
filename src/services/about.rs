//! Templates for "about" static pages.
#[must_use]
pub fn index_page() -> String {
    page()
}

#[must_use]
pub fn page() -> String {
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
            <a href="/info/license">license</a>
        </footer>
    </html>
"#
    .to_string()
}

#[must_use]
pub fn license_page() -> String {
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
            <a href="/info/about">about</a>
        </footer>
    </html>
"#
    .to_string()
}
