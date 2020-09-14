use nix::unistd::geteuid;

pub(crate) fn is_user_admin() -> {
    geteuid().is_root()
}