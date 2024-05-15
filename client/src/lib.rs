use serde::{Deserialize, Serialize};
mod settings;


#[derive(Deserialize, Serialize, Debug)]
struct ServiceResponse {
    uuid: String,
}


pub mod discovery_client {
    use crate::ServiceResponse;
    use gethostname::gethostname;
    use local_ip_address::local_ip;
    use tokio_cron_scheduler::{Job, JobScheduler};
    use crate::settings;

    static mut UUID : Option<String> = None;

    pub async fn init() {
        let settings = match settings::ScoutQuestConfig::new() {
            Ok(settings) => settings,
            Err(e) => panic!("Error loading settings: {}", e)
        };
        let ip_addr = match local_ip() {
            Ok(ip_addr) => ip_addr,
            Err(e) => panic!("Error getting local ip address: {}", e)
        };
        let hostname = gethostname().into_string().unwrap();
        println!("{:?}", settings);

        let client = reqwest::Client::new();
        let map = serde_json::json!({
            "name": settings.scout_quest_config.service_name.replace(" ", "_").to_uppercase(),
            "ip_addr": ip_addr,
            "hostname": hostname,
            "port": 3001
        });
        let url = format!("{}/api/services", settings.scout_quest_config.uri);
        match client.post(url)
            .json(&map)
            .send()
            .await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let resp = resp.json::<ServiceResponse>().await.unwrap();
                    unsafe {
                        UUID = Some(resp.uuid);
                    }
                    update_status().await;
                } else {
                    panic!("Error registering service: {}", resp.status());
                }
            
            },
            Err(e) => {
                panic!("Error registering service: {}", e);
            }
        };

        let sched = JobScheduler::new().await;
        let sched = match sched {
            Ok(sched) => sched,
            Err(e) => panic!("Can not initialized scheduler: {}", e)
        };
        let job = match Job::new_async("1/30 * * * * *", |_uuid, _l| {
            Box::pin(async move {
                update_status().await;
            })
        }) {
            Ok(job) => job,
            Err(_) => panic!("Failed to create job")
        };
        let _ = sched.add(job).await;
        sched.start().await.expect("Start scheduler failed");
    }

    async fn update_status() {
        let settings = match settings::ScoutQuestConfig::new() {
            Ok(settings) => settings,
            Err(e) => panic!("Error loading settings: {}", e)
        };
        let client = reqwest::Client::new();
        let uuid = match unsafe { UUID.clone() } {
            Some(uuid) => uuid,
            None => panic!("UUID not set")
        };
        let url = format!("{}/api/services/{}?status=Up", settings.scout_quest_config.uri, uuid);
        match client.put(url)
            .send()
            .await {
            Ok(_) => (),
            Err(e) => {
                panic!("Error updating service status: {}", e);
            }
        };
    }
}