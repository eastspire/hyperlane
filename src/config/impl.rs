use super::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            log_dir: DEFAULT_LOG_DIR,
            log_size: DEFAULT_LOG_FILE_SIZE,
            inner_print: DEFAULT_INNER_PRINT,
            inner_log: DEFAULT_INNER_LOG,
            websocket_buffer_size: DEFAULT_BUFFER_SIZE,
            http_line_buffer_size: DEFAULT_BUFFER_SIZE,
            nodelay: DEFAULT_NODELAY,
            linger: DEFAULT_LINGER,
            ttl: DEFAULT_TTI,
        }
    }
}
