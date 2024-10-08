pub struct ReqClient {
    inner: reqwest::Client,

    header: Vec<(String, String)>,

    line: String,

    host: String,

    hosts: String,

    mode: String,

    ipv4: String,

    method: String,

    referer: String,

    useragent: String,

    cookies: String,

    redirect: i32,

    dns_type: String,

    dns_server: String,

    key: String,

    beg: usize,

    end: usize,

    debug: bool,
}

impl ReqClient {
    pub fn new(cli: reqwest::Client, key: impl Into<String>, host: impl Into<String>) -> Self {
        let host = host.into();

        Self {
            inner: cli,
            header: vec![(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            )],
            line: Default::default(),
            host: host.clone(),
            hosts: host,
            mode: "fast".to_string(),
            ipv4: Default::default(),
            method: "get".to_string(),
            referer: Default::default(),
            useragent: Default::default(),
            cookies: Default::default(),
            redirect: 5,
            dns_type: "isp".to_string(),
            dns_server: Default::default(),
            key: key.into(),
            beg: 8,
            end: 24,
            debug: false,
        }
    }

    pub fn cli(&self) -> &reqwest::Client {
        &self.inner
    }

    pub fn header(&mut self, header: Vec<(String, String)>) -> &mut Self {
        self.header = header;
        self
    }

    pub fn key(&mut self, value: String) -> &mut Self {
        self.key = value;
        self
    }

    pub fn beg(&mut self, value: usize) -> &mut Self {
        self.beg = value;
        self
    }

    pub fn end(&mut self, value: usize) -> &mut Self {
        self.end = value;
        self
    }

    pub fn with_header(mut self, header: Vec<(String, String)>) -> Self {
        self.header = header;
        self
    }

    pub fn with_key(mut self, value: String) -> Self {
        self.key = value;
        self
    }

    pub fn with_beg(mut self, value: usize) -> Self {
        self.beg = value;
        self
    }

    pub fn with_end(mut self, value: usize) -> Self {
        self.end = value;
        self
    }

    // Set api, automate generated by api-gen ...
    pub fn line(&mut self, value: String) -> &mut Self {
        self.line = value;
        self
    }

    pub fn host(&mut self, value: String) -> &mut Self {
        self.host.clone_from(&value);
        self.hosts = value;
        self
    }

    pub fn mode(&mut self, value: String) -> &mut Self {
        self.mode = value;
        self
    }

    pub fn ipv4(&mut self, value: String) -> &mut Self {
        self.ipv4 = value;
        self
    }

    pub fn method(&mut self, value: String) -> &mut Self {
        self.method = value;
        self
    }

    pub fn referer(&mut self, value: String) -> &mut Self {
        self.referer = value;
        self
    }

    pub fn useragent(&mut self, value: String) -> &mut Self {
        self.useragent = value;
        self
    }

    pub fn cookies(&mut self, value: String) -> &mut Self {
        self.cookies = value;
        self
    }

    pub fn redirect(&mut self, value: i32) -> &mut Self {
        self.redirect = value;
        self
    }

    pub fn dns_type(&mut self, value: String) -> &mut Self {
        self.dns_type = value;
        self
    }

    pub fn dns_server(&mut self, value: String) -> &mut Self {
        self.dns_server = value;
        self
    }

    pub fn debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }

    // With api, automate generated by api-gen ...
    pub fn with_line(mut self, value: String) -> Self {
        self.line = value;
        self
    }

    pub fn with_host(mut self, value: String) -> Self {
        self.host.clone_from(&value);
        self.hosts = value;
        self
    }

    pub fn with_mode(mut self, value: String) -> Self {
        self.mode = value;
        self
    }

    pub fn with_ipv4(mut self, value: String) -> Self {
        self.ipv4 = value;
        self
    }

    pub fn with_method(mut self, value: String) -> Self {
        self.method = value;
        self
    }

    pub fn with_referer(mut self, value: String) -> Self {
        self.referer = value;
        self
    }

    pub fn with_useragent(mut self, value: String) -> Self {
        self.useragent = value;
        self
    }

    pub fn with_cookies(mut self, value: String) -> Self {
        self.cookies = value;
        self
    }

    pub fn with_redirect(mut self, value: i32) -> Self {
        self.redirect = value;
        self
    }

    pub fn with_dns_type(mut self, value: String) -> Self {
        self.dns_type = value;
        self
    }

    pub fn with_dns_server(mut self, value: String) -> Self {
        self.dns_server = value;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub async fn req_wssocket_msg(&self, url: &str) -> color_eyre::Result<String> {
        let mut req_builder = self.inner.post(url);

        for (key, value) in self.header.iter() {
            req_builder = req_builder.header(key, value);
        }
        let body = std::iter::zip(
            [
                "line",
                "host",
                "host_s",
                "check_mode",
                "ipv4",
                "method",
                "referer",
                "ua",
                "cookies",
                "redirect_num",
                "dns_server_type",
                "dns_server",
            ],
            [
                &self.line,
                &self.host,
                &self.hosts,
                &self.mode,
                &self.ipv4,
                &self.method,
                &self.referer,
                &self.useragent,
                &self.cookies,
                &self.redirect.to_string(),
                &self.dns_type,
                &self.dns_server,
            ],
        )
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&");

        if self.debug {
            eprintln!("Body: {body}");
        }

        req_builder = req_builder.body(body);
        // sending http request
        let resp = req_builder.send().await?;
        let html = resp.text().await?;

        // find task id in result
        let task_id = Self::find_task_id(&html, "task_id=")
            .ok_or_else(|| color_eyre::eyre::eyre!("Can not find task_id in result page"))?;

        if self.debug {
            eprintln!("Got task id = {}", task_id);
        }

        // cacluate the md5
        let md5 = Self::generate_md5(task_id, &self.key);

        if self.debug {
            eprintln!("Got md5 of $taskid$key = {}", md5);
        }

        let token = md5
            .get(self.beg..self.end)
            .ok_or_else(|| color_eyre::eyre::eyre!("Out of range, md5 string = `{}`", md5))?;

        Ok(format!(
            "{{\"task_id\":\"{}\",\"task_token\":\"{}\"}}",
            task_id, token
        ))
    }

    pub fn find_task_id<'a>(html: &'a str, pattern: &str) -> Option<&'a str> {
        html.find(pattern).and_then(|pos| {
            let left = html.get(pos + pattern.len()..)?.strip_prefix('\'')?;

            left.get(0..left.find('\'')?)
        })
    }

    pub fn generate_md5(task_id: &str, key: &str) -> String {
        use md5::Digest;

        let mut hasher = md5::Md5::new();

        hasher.update(task_id);
        hasher.update(key);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Message {
    ip: String,

    http_code: i32,

    all_time: String,

    dns_time: String,

    connect_time: String,

    download_time: String,

    redirect: i32,

    redirect_time: String,

    name: String,
}

impl Message {
    // Get api, automate generated by api-gen ...
    pub fn ip(&self) -> &str {
        self.ip.as_ref()
    }

    pub fn http_code(&self) -> i32 {
        self.http_code
    }

    pub fn all_time(&self) -> &str {
        self.all_time.as_ref()
    }

    pub fn dns_time(&self) -> &str {
        self.dns_time.as_ref()
    }

    pub fn connect_time(&self) -> &str {
        self.connect_time.as_ref()
    }

    pub fn download_time(&self) -> &str {
        self.download_time.as_ref()
    }

    pub fn redirect(&self) -> i32 {
        self.redirect
    }

    pub fn redirect_time(&self) -> &str {
        self.redirect_time.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn construct_row(&self) -> Vec<String> {
        [
            &self.name,
            &self.ip,
            &self.http_code.to_string(),
            &self.all_time,
            &self.dns_time,
            &self.connect_time,
            &self.download_time,
            &self.redirect.to_string(),
            &self.redirect_time,
        ]
        .map(String::from)
        .to_vec()
    }

    pub fn construct_header() -> Vec<String> {
        [
            "名称",
            "IP",
            "状态",
            "总时间",
            "DNS时间",
            "连接时间",
            "下载时间",
            "重定向",
            "重定向时间",
        ]
        .map(String::from)
        .to_vec()
    }
}
