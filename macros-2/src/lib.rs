pub trait ConfigDefault {
    fn get_default() -> Self;
}

#[derive(Debug)]
pub struct ConnectionTimeout(pub u64);
pub struct MaxConnections(pub u32);
pub struct RetryAttempts(pub u8);
pub struct PostgresPort(pub u16);
pub struct MySQLPort(pub u16);
pub struct MongoPort(pub u16);
pub struct RedisPort(pub u16);

#[macro_export]
macro_rules! config_default_impl {
    ($type:ty, $value:expr) => {
        impl ConfigDefault for $type {
            fn get_default() -> Self {
                Self($value)
            }
        }
    };

    ($type:ty, $value:expr, $($tail_type:ty, $tail_value:expr),+) => {
        config_default_impl!($type, $value);
        config_default_impl!($($tail_type, $tail_value),+);
    };
}

config_default_impl!(ConnectionTimeout, 30, MaxConnections, 100);
config_default_impl!(RetryAttempts, 3, PostgresPort, 5432);
config_default_impl!(MySQLPort, 3306, MongoPort, 27017, RedisPort, 6379);

pub fn main() {
    // let's say we have a new struct
    struct CustomPort(pub u16);

    // we implement the ConfigDefault trait for CustomPort
    config_default_impl!(CustomPort, 8080);

    // when running the `get_default` method, it should return the default value
    assert_eq!(<CustomPort as ConfigDefault>::get_default().0, 8080);
}
