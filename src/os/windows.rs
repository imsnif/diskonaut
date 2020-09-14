
use winapi::um::winnt::{
    PVOID, SECURITY_BUILTIN_DOMAIN_RID,
    SECURITY_NT_AUTHORITY, SID_IDENTIFIER_AUTHORITY, 
    DOMAIN_ALIAS_RID_ADMINS
};

use winapi::um::securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership};

pub(crate) fn is_user_admin() -> bool {
    let mut auth_nt = SID_IDENTIFIER_AUTHORITY {
        Value: SECURITY_NT_AUTHORITY,
    };
    let mut admingroup = 0 as PVOID;
    let ismember = unsafe {
        assert!(AllocateAndInitializeSid(
            &mut auth_nt,
            2,
            SECURITY_BUILTIN_DOMAIN_RID,
            DOMAIN_ALIAS_RID_ADMINS,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut admingroup,
        ) != 0);
        let mut member:i32 = 0;
        assert!(CheckTokenMembership(0 as PVOID,admingroup, &mut member) != 0);
        member != 0

    };
    ismember
}
