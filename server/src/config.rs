use std::env;
use config_rs::{ConfigError, Config, File};
use aws_sdk_s3::{Client,Endpoint};
use aws_sdk_s3::Config as s3_config;
use http::{Uri};

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    pub s3: S3,
}

#[derive(Debug,Deserialize)]
pub struct S3 {
    aws_access_key_id: String,
    aws_secret_key:  String,
    aws_default_region:  String,
    aws_endpoint_url: String,
    aws_bucket_name:  String,
}

impl ConfigToml {
    pub fn new() -> ConfigToml {
        let c = ConfigToml::new_config();
        return c.unwrap();
    }
    pub fn new_config() -> Result<ConfigToml, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("Config"))?;
        s.try_into()
    }
}

pub trait Aws {
    fn s3client(&self) -> aws_sdk_s3::Client;
}

impl Aws for S3 {
    fn s3client(&self) -> aws_sdk_s3::Client {
        let ( secret_key, access_key, region, endpoint_url, bucket_name ) = ("AWS_SECRET_ACCESS_KEY", "AWS_ACCESS_KEY_ID", "AWS_DEFAULT_REGION", "AWS_ENDPOINT_URL", "AWS_BUCKET_NAME");
        let  data = vec![secret_key, access_key, region, endpoint_url, bucket_name];

        for data in data.iter() {
            match data {
                &"AWS_SECRET_ACCESS_KEY" => env::set_var(secret_key, &self.aws_secret_key),
                &"AWS_ACCESS_KEY_ID" => env::set_var(access_key, &self.aws_access_key_id),
                &"AWS_DEFAULT_REGION" => env::set_var(region, &self.aws_default_region),
                &"AWS_ENDPOINT_URL" => env::set_var(endpoint_url, &self.aws_endpoint_url),
                &"AWS_BUCKET_NAME" => env::set_var(bucket_name, &self.aws_bucket_name),
                _ => println!("{:?}",&self),
            }
            
        }
        let ep = env::var("AWS_ENDPOINT_URL").unwrap_or("none".to_string());
        let uri = ep.parse::<Uri>().unwrap();
        let endpoint = Endpoint::immutable(uri);
        let config = s3_config::builder().endpoint_resolver(endpoint).build();
        Client::from_conf(config)
    }
}