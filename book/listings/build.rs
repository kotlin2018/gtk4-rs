use gtk::gio;

fn main() {
    // actions
    gio::compile_resources(
        "actions/5/resources",
        "actions/5/resources/resources.gresource.xml",
        "actions_5.gresource",
    );
    gio::compile_resources(
        "actions/6/resources",
        "actions/6/resources/resources.gresource.xml",
        "actions_6.gresource",
    );
    gio::compile_resources(
        "actions/7/resources",
        "actions/7/resources/resources.gresource.xml",
        "actions_7.gresource",
    );

    // composite_templates
    gio::compile_resources(
        "composite_templates/1/resources",
        "composite_templates/1/resources/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
    gio::compile_resources(
        "composite_templates/2/resources",
        "composite_templates/2/resources/resources.gresource.xml",
        "composite_templates_2.gresource",
    );
    gio::compile_resources(
        "composite_templates/3/resources",
        "composite_templates/3/resources/resources.gresource.xml",
        "composite_templates_3.gresource",
    );
    gio::compile_resources(
        "composite_templates/4/resources",
        "composite_templates/4/resources/resources.gresource.xml",
        "composite_templates_4.gresource",
    );
    gio::compile_resources(
        "composite_templates/5/resources",
        "composite_templates/5/resources/resources.gresource.xml",
        "composite_templates_5.gresource",
    );
    gio::compile_resources(
        "composite_templates/6/resources",
        "composite_templates/6/resources/resources.gresource.xml",
        "composite_templates_6.gresource",
    );

    // todo_app
    gio::compile_resources(
        "todo_app/1/resources",
        "todo_app/1/resources/resources.gresource.xml",
        "todo_app_1.gresource",
    );
    gio::compile_resources(
        "todo_app/2/resources",
        "todo_app/2/resources/resources.gresource.xml",
        "todo_app_2.gresource",
    );
    gio::compile_resources(
        "todo_app/3/resources",
        "todo_app/3/resources/resources.gresource.xml",
        "todo_app_3.gresource",
    );
    gio::compile_resources(
        "todo_app/4/resources",
        "todo_app/4/resources/resources.gresource.xml",
        "todo_app_4.gresource",
    );
}
