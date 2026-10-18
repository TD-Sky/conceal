use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        freedesktop: {
            all(
                unix,
                not(target_os = "macos"),
                not(target_os = "ios"),
                not(target_os = "android")
        )}
    }
}
