use anyhow::Result;
use sf_express::SfExpress;
use std::env;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    if let Ok(env_val) = env::var("SF_EXPRESS_SESSION_ID") {
        let items = env_val
            .split(';')
            .filter(|f| !f.is_empty())
            .collect::<Vec<&str>>();
        println!("已配置{}个SESSION_ID, 开始执行任务!", items.len());
        for item in items {
            if let Ok(sf_express) = SfExpress::new(item.to_string()).await {
                sf_express.run().await.ok();
            } else {
                println!("请检查SESSION_ID:{}", item);
            }
        }
    }else{
        println!("未配置SESSION_ID, 退出...");
    }
    Ok(())
}
