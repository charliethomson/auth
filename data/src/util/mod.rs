use sea_orm::{ActiveValue, sea_query::Nullable};

pub trait IntoActiveValueExt {
    type Source;
    fn into_active_value_ext<Target>(self) -> ActiveValue<Target>
    where
        Target: From<Self::Source>,
        sea_orm::Value: From<Target>;
    fn into_active_value_opt_ext<Target>(self) -> ActiveValue<Option<Target>>
    where
        Target: From<Self::Source> + Nullable,
        sea_orm::Value: From<Target>;
}

impl<T> IntoActiveValueExt for Option<T> {
    type Source = T;

    fn into_active_value_ext<Target>(self) -> ActiveValue<Target>
    where
        Target: From<Self::Source>,
        sea_orm::Value: From<Target>,
    {
        match self {
            Some(source) => ActiveValue::Set(source.into()),
            None => ActiveValue::NotSet,
        }
    }

    fn into_active_value_opt_ext<Target>(self) -> ActiveValue<Option<Target>>
    where
        Target: From<Self::Source> + Nullable,
        sea_orm::Value: From<Target>,
    {
        match self {
            Some(source) => ActiveValue::Set(Some(source.into())),
            None => ActiveValue::NotSet,
        }
    }
}
