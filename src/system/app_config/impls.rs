use config::{Config, ConfigError, Environment, File};
use std::sync::OnceLock;

use super::AppConfig;

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    /// 加载配置
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            // 首先加载默认配置文件
            .add_source(File::with_name("config").required(false))
            // 然后根据环境加载特定配置文件
            .add_source(
                File::with_name(&format!(
                    "config.{}",
                    std::env::var("APP_ENV").unwrap_or_else(|_| "development".into())
                ))
                .required(false),
            )
            // 最后加载环境变量覆盖
            .add_source(
                Environment::with_prefix("HWSYSTEM")
                    .separator("_")
                    .try_parsing(true),
            );

        // 支持从环境变量加载
        builder = builder
            .set_override_option("app.environment", std::env::var("APP_ENV").ok())?
            .set_override_option("app.log_level", std::env::var("RUST_LOG").ok())?
            .set_override_option("server.host", std::env::var("SERVER_HOST").ok())?
            .set_override_option("server.port", std::env::var("SERVER_PORT").ok())?
            .set_override_option("server.unix_socket_path", std::env::var("UNIX_SOCKET").ok())?
            .set_override_option("server.workers", std::env::var("CPU_COUNT").ok())?
            .set_override_option("jwt.secret", std::env::var("JWT_SECRET").ok())?
            .set_override_option("database.backend", std::env::var("STORAGE_BACKEND").ok())?
            .set_override_option("database.url", std::env::var("DATABASE_URL").ok())?
            .set_override_option("cache.redis.url", std::env::var("REDIS_URL").ok())?
            .set_override_option(
                "cache.redis.key_prefix",
                std::env::var("REDIS_KEY_PREFIX").ok(),
            )?
            .set_override_option("cache.redis.default_ttl", std::env::var("REDIS_TTL").ok())?;

        let config = builder.build()?;
        let mut app_config: AppConfig = config.try_deserialize()?;

        // 处理工作线程数
        if app_config.server.workers == 0 {
            app_config.server.workers = num_cpus::get().min(app_config.server.max_workers);
        }

        Ok(app_config)
    }

    /// 获取全局配置实例
    pub fn get() -> &'static AppConfig {
        APP_CONFIG.get_or_init(|| {
            Self::load().unwrap_or_else(|e| {
                eprintln!("Failed to load configuration: {e}");
                std::process::exit(1);
            })
        })
    }

    /// 初始化配置 (在应用启动时调用)
    pub fn init() -> Result<(), ConfigError> {
        let config = Self::load()?;
        APP_CONFIG
            .set(config)
            .map_err(|_| ConfigError::Message("Configuration already initialized".to_string()))?;
        Ok(())
    }

    /// 检查是否为生产环境
    pub fn is_production(&self) -> bool {
        self.app.environment == "production"
    }

    /// 检查是否为开发环境
    pub fn is_development(&self) -> bool {
        self.app.environment == "development"
    }

    /// 获取服务器绑定地址
    pub fn server_bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// 获取 Unix 套接字路径 (如果配置了)
    #[cfg(unix)]
    pub fn unix_socket_path(&self) -> Option<&str> {
        if self.server.unix_socket_path.is_empty() {
            None
        } else {
            Some(&self.server.unix_socket_path)
        }
    }
}
