use anyhow::{anyhow, Result};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::{json, Value};
use std::time::Duration;

pub struct SfExpress {
    client: Client,
    device_id: String,
    nickname: String,
    domain: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    title: String,
    strategy_id: u16,
    status: u8,
    point: u8,
    task_id: String,
    task_code: String,
}

impl SfExpress {
    pub async fn new(session_id: String) -> Result<SfExpress> {
        let client_builder = Client::builder();
        let mut headers = HeaderMap::new();
        headers.append(
            "origin",
            HeaderValue::from_str("https://mcs-mimp-web.sf-express.com")?,
        );
        headers.append("platform", HeaderValue::from_str("SFAPP")?);
        headers.append("MCS-MIMP-CORE", HeaderValue::from_str("MCS-MIMP-CORE")?);
        headers.append(
            "user-agent",
            HeaderValue::from_str("Mozilla/5.0 (iPhone; CPU iPhone OS 16_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 mediaCode=SFEXPRESSAPP-iOS-ML")?,
        );
        headers.append(
            "cookie",
            HeaderValue::from_str(&format!("sessionId={};", session_id))?,
        );
        let client = client_builder.default_headers(headers).build()?;
        let domain = "https://mcs-mimp-web.sf-express.com";
        let url = format!(
            "{}/mcs-mimp/commonPost/~memberIntegral~userInfoService~queryUserInfo",
            domain
        );
        let resp = client
            .post(url)
            .json(&json!({
                "sysCode": "ESG-CEMP-CORE",
                "optionalColumns": ["usablePoint", "cycleSub", "leavePoint"],
                "token": "zeTLTYeG0bLetfRk"
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;
        let device_id_prefix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let device_id_infix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();
        let device_id_suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();
        let device_id = format!(
            "{}-{}-{}",
            device_id_prefix, device_id_infix, device_id_suffix
        );
        let is_success = resp
            .get("success")
            .unwrap_or(&Value::Bool(false))
            .as_bool()
            .unwrap();
        match is_success {
            true => {
                let nickname = resp["obj"]["nickName"].as_str().unwrap();
                Ok(Self {
                    client,
                    nickname: nickname.into(),
                    domain: domain.into(),
                    device_id,
                })
            }
            false => Err(anyhow!("登录失败, 请检查SESSION_ID是否有效, {}", resp)),
        }
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>> {
        let url = format!("{}/mcs-mimp/commonPost/~memberNonactivity~integralTaskStrategyService~queryPointTaskAndSignFromES", self.domain);

        let resp = self
            .client
            .post(url)
            .json(&json!({
                "channelType": "1",
                "deviceId": "b85a7d52-4cde-a2d4"
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;
        let is_success = resp
            .get("success")
            .unwrap_or(&Value::Bool(false))
            .as_bool()
            .unwrap();
        let task_list = match is_success {
            true => {
                let tasks = resp["obj"]["taskTitleLevels"].clone();
                serde_json::from_value::<Vec<Task>>(tasks).unwrap()
            }
            false => Vec::new(),
        };
        Ok(task_list)
    }

    pub async fn get_award(&self, task_list: Vec<Task>) -> Result<()> {
        let award_url = format!(
            "{}/mcs-mimp/commonPost/~memberNonactivity~integralTaskStrategyService~fetchIntegral",
            self.domain
        );
        for task in task_list {
            if task.status == 1 {
                // 1: 待领取奖励
                // 待领奖
                let resp = self
                    .client
                    .post(award_url.clone())
                    .json(&json!({
                        "strategyId": task.strategy_id,
                        "taskId": task.task_id,
                        "taskCode": task.task_code,
                        "deviceId": self.device_id,
                    }))
                    .send()
                    .await?
                    .json::<Value>()
                    .await?;
                let is_success = resp
                    .get("success")
                    .unwrap_or(&Value::Bool(false))
                    .as_bool()
                    .unwrap();
                if is_success {
                    println!(
                        "{}, 成功领取任务:《{}》奖励, 获得积分:{}!",
                        self.nickname, task.title, task.point
                    );
                    tokio::time::sleep(Duration::from_secs(3)).await;
                } else {
                    println!("{}, 领取任务《{}》奖励失败!", self.nickname, task.title);
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            }else{
                println!("{}, 今日已领取任务:《{}》奖励!", self.nickname, task.title);
            }
        }
        Ok(())
    }

    pub async fn sign(&self) -> Result<()> {
        let can_sign_url = format!("{}/mcs-mimp/commonPost/~memberNonactivity~integralTaskSignPlusService~automaticSignFetchPackage", self.domain);
        let resp = self.client.post(can_sign_url).json(&json!({
            "channelFrom": "SFAPP"
        })).send().await?.json::<Value>().await?;
        let is_success = resp
        .get("success")
        .unwrap_or(&Value::Bool(false))
        .as_bool()
        .unwrap();
        match is_success {
            true => {
                let sign_days = resp["obj"]["countDay"].as_u64().unwrap_or(0);
                println!("{}, 今日已签到, 连续签到天数:{}", self.nickname, sign_days);
            },
            false => {
                println!("{}, 获取签到信息失败!", self.nickname);
            },
        }
        Ok(())
    }

    pub async fn query_point(&self) -> Result<()> {
        let url = format!("{}/mcs-mimp/commonPost/~memberNonactivity~integralTaskStrategyService~queryPointTaskAndSignFromES", self.domain);

        let resp = self
            .client
            .post(url)
            .json(&json!({
                "channelType": "1",
                "deviceId": "b85a7d52-4cde-a2d4"
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;
        let is_success = resp
            .get("success")
            .unwrap_or(&Value::Bool(false))
            .as_bool()
            .unwrap();
        match is_success {
            true => {
                let points = resp["obj"]["totalPoint"].as_u64().unwrap_or(0);
                println!("{}, 当前已有积分:{}", self.nickname, points);
            },
            false => {
                println!("{}, 查询积分数据失败!", self.nickname)
            },
        }
        Ok(())
    }

    pub async fn do_tasks(&self, task_list: Vec<Task>) -> Result<()> {
        let task_url = format!(
            "{}/mcs-mimp/commonPost/~memberEs~taskRecord~finishTask",
            self.domain
        );

        for task in task_list {
            if task.status != 2 {
                //  2:待做任务
                continue;
            }
            if task.title.contains("完成每月任务")
                || task.title.contains("参与积分活动")
                || task.title.contains("每月累计寄件")
                || task.title.contains("添加顺丰小组件")
            {
                println!("{}, 跳过任务:《{}》!", self.nickname, task.title);
                continue;
            }
            let resp = self
                .client
                .post(task_url.clone())
                .json(&json!({
                    "taskCode": task.task_code,
                }))
                .send()
                .await?
                .json::<Value>()
                .await?;
            let is_success = resp
                .get("obj")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap();
            if is_success {
                println!("{}, 成功完成任务:《{}》!", self.nickname, task.title);
                tokio::time::sleep(Duration::from_secs(3)).await;
            } else {
                println!("{}, 无法完成任务:《{}》!", self.nickname, task.title);
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        self.sign().await?;
        let task_list = self.get_tasks().await?;
        self.do_tasks(task_list).await?;
        let task_list = self.get_tasks().await?;
        self.get_award(task_list).await?;
        self.query_point().await?;
        Ok(())
    }
}