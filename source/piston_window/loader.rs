fn main() {
    settings := {
        background_color: [1; 4],
        reload_key: 1073741882, // F1
    }
    source := "source/piston_window/square.rs"
    m := unwrap(load(source))

    set(title: call_ret(m, "title", []))

    time := 0
    last_reload := 0
    reload_interval := 0.25
    got_error := false

    loop {
        if !next_event() { break }
        if render() {
            call(m, "render", [settings])
        }
        if update() {
            dt := unwrap(update_dt())
            time += dt
            if !got_error && ((last_reload + reload_interval) < time) {
                last_reload = clone(time)
                new_m := load(source)
                if is_err(new_m) {
                    got_error = true
                    println(unwrap_err(new_m))
                    println(" ~~~ Hit F1 to reload ~~~ ")
                } else {
                    got_error = false
                    m = unwrap(new_m)
                }
            }
        }
        if press() {
            key := press_keyboard_key()
            if key == some(settings.reload_key) {
                println(" ~~~ Reloading ~~~ ")
                got_error = false
            }
        }
    }
}
