use std::sync::OnceLock;

use minijinja::Environment;

fn jinja_env() -> &'static Environment<'static> {
    static ENV: OnceLock<Environment> = OnceLock::new();
    ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.add_template("page.jinja", include_str!("../templates/page.jinja"))
            .expect("page template should parse");
        env.add_template(
            "message_block.jinja",
            include_str!("../templates/message_block.jinja"),
        )
        .expect("message block template should parse");
        env.add_template("error.jinja", include_str!("../templates/error.jinja"))
            .expect("error template should parse");
        env
    })
}

pub(crate) fn render_template(name: &str, ctx: minijinja::Value) -> String {
    jinja_env()
        .get_template(name)
        .unwrap_or_else(|_| panic!("template {name} should exist"))
        .render(ctx)
        .unwrap_or_else(|e| panic!("failed to render template {name}: {e}"))
}
