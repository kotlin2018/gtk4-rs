# Saving Window State

Quite often, we want the window state to persist between sessions.
If the user resizes or maximizes the window, they might expect to find it in the same state the next time they open the app.
GTK does not provide this functionality out of the box, but luckily it is not too hard to manually implement it.
We basically want two integers (`height` & `width`) and a boolean (`is_maximized`) to persist.
We already know how to do this by using [`gio::Settings`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Settings.html).

<span class="filename">Filename: listings/saving_window_state/1/org.gtk-rs.example.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/saving_window_state/1/org.gtk-rs.example.gschema.xml}}
```

Since we don't care about intermediate state, we only load the window state when the window is constructed and save it when we close the window.
That can be done by creating a custom window.
First, we create one and add methods for getting and setting the window state.

<span class="filename">Filename: listings/saving_window_state/1/custom_window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/saving_window_state/1/custom_window/mod.rs:mod}}
```

> We set the property "application" by passing it to [`glib::Object::new`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.Object.html#method.new).
> You can even set multiple properties that way.
> When creating new GObjects, this is nicer than calling the setter methods manually.

The implementation struct holds the `settings`.
We also override the `constructed` and `close_request` methods, where we load or save the window state. 

<span class="filename">Filename: listings/saving_window_state/1/custom_window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/saving_window_state/1/custom_window/imp.rs:imp}}
```

That is it!
Now our window retains its state between app sessions.
