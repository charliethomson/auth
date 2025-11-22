use std::collections::HashSet;

use strum::{Display, EnumString};

use crate::services::core::jwt::Claims;

#[derive(EnumString, Display)]
pub enum Grants {
    #[strum(to_string = "dev.thmsn.auth.user.create")]
    UserCreate,
    #[strum(to_string = "dev.thmsn.auth.user.delete")]
    UserDelete,
    #[strum(to_string = "dev.thmsn.auth.user.list")]
    UserList,
    #[strum(to_string = "dev.thmsn.auth.user.get")]
    UserGet,
    #[strum(to_string = "dev.thmsn.auth.user.grant.update")]
    UserGrantUpdate,
    #[strum(to_string = "dev.thmsn.auth.application.create")]
    ApplicationCreate,
    #[strum(to_string = "dev.thmsn.auth.application.get")]
    ApplicationGet,
    #[strum(to_string = "dev.thmsn.auth.application.list")]
    ApplicationList,
    #[strum(to_string = "dev.thmsn.auth.application.get_grants")]
    ApplicationGetGrants,
    #[strum(to_string = "dev.thmsn.auth.application.update")]
    ApplicationUpdate,
    #[strum(to_string = "dev.thmsn.auth.grant.create")]
    GrantCreate,
    #[strum(to_string = "dev.thmsn.auth.grant.get")]
    GrantGet,
}

#[derive(Default, Debug)]
#[allow(unused)]
pub enum HasGrantsMode {
    #[default]
    And,
    Or,
}

pub trait HasGrants {
    type Grants: ToString;

    fn get_grants(&self) -> HashSet<&str>;

    fn has_grants(&self, grants: &[Self::Grants]) -> bool {
        self.has_grants_pro(grants, HasGrantsMode::default())
    }
    fn has_grants_pro(&self, grants: &[Self::Grants], mode: HasGrantsMode) -> bool {
        let user_grants = self.get_grants();

        for grant in grants {
            let it = grant.to_string();
            match mode {
                HasGrantsMode::And if !user_grants.contains(it.as_str()) => return false,
                HasGrantsMode::Or if user_grants.contains(it.as_str()) => return true,
                _ => {}
            }
        }

        match mode {
            HasGrantsMode::And => true,
            HasGrantsMode::Or => false,
        }
    }
}

impl HasGrants for Claims {
    type Grants = Grants;
    fn get_grants(&self) -> HashSet<&str> {
        self.grants.iter().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::util::grants::{HasGrants, HasGrantsMode};

    struct FakeClaims {
        grants: Vec<String>,
    }

    impl HasGrants for FakeClaims {
        type Grants = &'static str;
        fn get_grants(&self) -> HashSet<&str> {
            self.grants.iter().map(|s| s.as_str()).collect()
        }
    }
    macro_rules! claims {
        [$($e:expr),*] => {
            FakeClaims {
                grants: vec![
                    $($e.into(),)*
                ]
            }
        };
    }

    #[test]
    fn test_has_grant_mode_default() {
        let claims = claims!["a", "b", "c"];

        assert_eq!(true, claims.has_grants(&["a"]));
        assert_eq!(true, claims.has_grants(&["a", "b"]));
        assert_eq!(true, claims.has_grants(&["a", "b", "c"]));
        assert_eq!(false, claims.has_grants(&["a", "b", "c", "d"]));
    }

    #[test]
    fn test_has_grant_mode_and() {
        let claims = claims!["a", "b", "c"];

        assert_eq!(true, claims.has_grants_pro(&["a"], HasGrantsMode::And));
        assert_eq!(true, claims.has_grants_pro(&["a", "b"], HasGrantsMode::And));
        assert_eq!(
            true,
            claims.has_grants_pro(&["a", "b", "c"], HasGrantsMode::And)
        );
        assert_eq!(
            false,
            claims.has_grants_pro(&["a", "b", "c", "d"], HasGrantsMode::And)
        );
    }
    #[test]
    fn test_has_grant_mode_or() {
        let claims = claims!["a", "b", "c"];

        assert_eq!(true, claims.has_grants_pro(&["a"], HasGrantsMode::Or));
        assert_eq!(true, claims.has_grants_pro(&["a", "b"], HasGrantsMode::Or));
        assert_eq!(
            true,
            claims.has_grants_pro(&["a", "b", "c"], HasGrantsMode::Or)
        );
        assert_eq!(
            true,
            claims.has_grants_pro(&["a", "b", "c", "d"], HasGrantsMode::Or)
        );
        assert_eq!(false, claims.has_grants_pro(&["d", "e"], HasGrantsMode::Or));
    }
}
