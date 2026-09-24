use crate::AppState;

struct WeatherDay {
    date: chrono::NaiveDate,
    week: Option<String>,
    high: f64,
    low: f64,
    morning: Option<String>,
    afternoon: Option<String>,
    wind: Option<String>,
    aqi: i32,
}

const ENDPOINT: &str = "https://tianqi.2345.com/Pc/GetHistory";

pub(crate) async fn sync_month(
    state: &AppState,
    area_id: &str,
    year: i32,
    month: u32,
) -> Result<usize, String> {
    let html = fetch_month(state, area_id, year, month).await?;
    let days = parse_history(&html)?;
    persist(state, area_id, year, month, &days).await
}

pub(crate) async fn probe_month(
    state: &AppState,
    area_id: &str,
    year: i32,
    month: u32,
) -> Result<usize, String> {
    let html = fetch_month(state, area_id, year, month).await?;
    parse_history(&html).map(|days| days.len())
}

async fn fetch_month(
    state: &AppState,
    area_id: &str,
    year: i32,
    month: u32,
) -> Result<String, String> {
    let socket = state
        .broker_socket
        .as_deref()
        .ok_or_else(|| "AIO broker socket 未提供，无法访问天气服务".to_owned())?;
    let query = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("areaInfo[areaId]", area_id)
        .append_pair("areaInfo[areaType]", "2")
        .append_pair("date[year]", &year.to_string())
        .append_pair("date[month]", &month.to_string())
        .finish();
    let endpoint = format!("{ENDPOINT}?{query}");
    let client = reqwest::Client::builder()
        .unix_socket(socket)
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|error| format!("创建 AIO broker 客户端失败：{error}"))?;
    let response = client
        .post("http://localhost/egress/http")
        .header("x-aio-token", &state.ingress_token)
        .header("x-aio-endpoint", &endpoint)
        .header("x-aio-method", "GET")
        .header("accept", "application/json, text/javascript, */*; q=0.01")
        .header("accept-language", "zh-CN,zh;q=0.9")
        .header(
            "referer",
            format!("https://tianqi.2345.com/wea_history/{area_id}.htm"),
        )
        .header(
            "user-agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36",
        )
        .header("x-requested-with", "XMLHttpRequest")
        .body("{}")
        .send()
        .await
        .map_err(|error| format!("天气出站请求失败：{error}"))?;
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("读取天气响应失败：{error}"))?;
    let text = decode_weather_body(&bytes);
    if !status.is_success() {
        return Err(format!(
            "天气服务返回 HTTP {status}：{}",
            text.chars().take(200).collect::<String>()
        ));
    }
    if text.contains("抱歉，暂无") {
        return Ok(String::new());
    }
    let payload: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| format!("天气响应不是 JSON：{error}"))?;
    Ok(payload
        .get("data")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned())
}

fn decode_weather_body(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .unwrap_or_else(|_| encoding_rs::GBK.decode(bytes).0.into_owned())
}

fn parse_history(html: &str) -> Result<Vec<WeatherDay>, String> {
    let table = html
        .split_once("<tbody")
        .and_then(|(_, rest)| rest.split_once("</tbody>").map(|(body, _)| body))
        .unwrap_or(html);
    let mut days = Vec::new();
    for row in table.split("<tr") {
        let cells = row
            .split("<td")
            .skip(1)
            .map(|cell| {
                let value = cell
                    .split_once('>')
                    .map(|(_, value)| value)
                    .unwrap_or(cell)
                    .split_once("</td>")
                    .map(|(value, _)| value)
                    .unwrap_or(cell);
                value
                    .replace(['\n', '\r'], "")
                    .replace("&nbsp;", " ")
                    .trim()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        if cells.len() < 4 {
            continue;
        }
        let mut date_parts = cells[0].split_whitespace();
        let date_text = date_parts.next().unwrap_or_default();
        let Some(date) = chrono::NaiveDate::parse_from_str(date_text, "%Y-%m-%d").ok() else {
            continue;
        };
        let high = number(&cells[1]);
        let low = number(&cells[2]);
        days.push(WeatherDay {
            date,
            week: date_parts.next().map(str::to_owned),
            high,
            low,
            morning: cells.get(3).cloned(),
            afternoon: None,
            wind: cells.get(4).cloned(),
            aqi: cells
                .get(5)
                .and_then(|value| value.split_whitespace().next())
                .and_then(|value| value.parse::<i32>().ok())
                .unwrap_or(0),
        });
    }
    Ok(days)
}

fn number(value: &str) -> f64 {
    value
        .chars()
        .filter(|character| character.is_ascii_digit() || *character == '-' || *character == '.')
        .collect::<String>()
        .parse()
        .unwrap_or(0.0)
}

async fn persist(
    state: &AppState,
    area_id: &str,
    year: i32,
    month: u32,
    days: &[WeatherDay],
) -> Result<usize, String> {
    let mut accumulated: f64 = 0.0;
    let mut affected = 0;
    for day in days {
        state
            .database
            .execute(
                "DELETE FROM boxun_historical_weather WHERE area_id=$1 AND weather_date=$2",
                &[&area_id, &day.date],
            )
            .await
            .map_err(|error| format!("清理旧天气记录失败：{error}"))?;
        let next_id: i64 = state
            .database
            .query_one(
                "SELECT COALESCE(MAX(id),0)+1 FROM boxun_historical_weather",
                &[],
            )
            .await
            .map_err(|error| format!("生成天气主键失败：{error}"))?
            .get(0);
        accumulated += (day.high + day.low) / 2.0;
        state
            .database
            .execute(
                "INSERT INTO boxun_historical_weather (create_time,id,weather_date,week,maximum_temperature,lowest_temperature,morning,afternoon,wind_direction,air_quality_index,accumulated_temperature,area_id,area_type) VALUES (CURRENT_TIMESTAMP,$1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,'2')",
                &[
                    &next_id,
                    &day.date,
                    &day.week,
                    &day.high,
                    &day.low,
                    &day.morning,
                    &day.afternoon,
                    &day.wind,
                    &day.aqi.to_string(),
                    &accumulated.round().to_string(),
                    &area_id,
                ],
            )
            .await
            .map_err(|error| format!("写入天气失败：{error}"))?;
        affected += 1;
    }
    let _ = (year, month);
    Ok(affected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_history_table() {
        let html = "<tbody><tr><td>2026-01-01 周三</td><td>10℃</td><td>-2℃</td><td>晴</td><td>东风 2级</td><td>50 优</td></tr></tbody>";
        let rows = parse_history(html).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].high, 10.0);
        assert_eq!(rows[0].low, -2.0);
    }
}
