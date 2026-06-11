#[derive(Debug)]
pub struct Query {
    pub order_by: String,
    pub signal: Option<String>,
    pub parameter: Option<String>,
    pub columns: Vec<u16>,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            order_by: "ticker".to_string(),
            columns: Vec::new(),
            signal: None,
            parameter: None,
        }
    }
}

impl Query {
    pub fn url(&self, auth: &str) -> String {
        let parameter = match &self.parameter {
            Some(p) => p.as_str(),
            None => "",
        };

        let signal = match &self.signal {
            Some(s) => s.as_str(),
            None => "",
        };

        let columns = self
            .columns
            .iter()
            .map(|&c| c.to_string())
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "https://elite.finviz.com/export?v=111&f={}&c={}&s={}&o={}&auth={}",
            parameter, columns, signal, self.order_by, auth
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::screener::Query;

    #[test]
    fn to_url() {
        let query_1 = Query::default();
        let query_2 = Query {
            columns: vec![0, 1, 2, 3],
            parameter: Some(String::from("fa_div_pos,sec_technology")),
            signal: Some(String::from("ta_unusualvolume")),
            ..Default::default()
        };

        println!("{}", query_1.url("xxx"));
        println!("{}", query_2.url("xxx"));
    }
}
