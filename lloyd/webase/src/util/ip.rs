use std::collections::HashMap;

pub fn find_inner_ip() -> String {
    for iface in pnet_datalink::interfaces() {
        for ip in iface.ips {
            let ip_str = ip.ip().to_string();
            if ip_str.starts_with("192") {
                return ip_str;
            }
            if ip_str.starts_with("172") {
                return ip_str;
            }
            if ip_str.starts_with("10") {
                return ip_str;
            }
        }
    }
    String::from("127.0.0.1")
}

pub async fn find_public_ip() -> Result<String, reqwest::Error> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let result = resp.get("origin").map(|i| i.to_string());
    Ok(result.unwrap_or_default())
}
