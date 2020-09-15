// for not windows - linux or mac
use nix::unistd::geteuid;

pub(crate) fn is_user_admin() -> bool {
    geteuid().is_root()
}
