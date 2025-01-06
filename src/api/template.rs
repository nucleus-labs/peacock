pub(crate) fn jinja_load() {}

pub(crate) fn jinja_include() {}

pub(crate) fn populate_env(_env: &mut minijinja::Environment<'_>) {
    // env.add_function("load", jinja_load);
}
