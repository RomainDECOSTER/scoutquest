use serde::{Deserialize, Serialize};
mod settings;


#[derive(Deserialize, Serialize, Debug)]
struct ServiceResponse {
    uuid: String,
}

/// discovery_client module
/// 
/// This module is responsible for registering the service to the server and updating the service status.
/// 
/// # Example
/// ```
/// use scoutquest_client::discovery_client::init;
/// 
/// #[tokio::main]
/// async fn main() {
///     init().await;
/// }
/// ```
/// # Note
/// This module uses the settings module to load the settings.
/// 
/// This module will start a scheduler to update the service status every 30 seconds.
/// 
/// # Panics
/// 
/// This module will panic if the settings can not be loaded, the local ip address can not be retrieved, the hostname can not be retrieved, the service can not be registered, the service status can not be updated, the scheduler can not be initialized, the job can not be created.
pub mod discovery_client {
    use crate::ServiceResponse;
    use gethostname::gethostname;
    use local_ip_address::local_ip;
    use tokio_cron_scheduler::{Job, JobScheduler};
    use crate::settings;

    static mut UUID : Option<String> = None;

    /// Initialize the discovery client
    /// 
    /// # Panics
    /// This function will panic if the settings can not be loaded, the local ip address can not be retrieved, the hostname can not be retrieved, the service can not be registered, the service status can not be updated, the scheduler can not be initialized, the job can not be created.
    /// 
    /// # Note
    /// This function will start a scheduler to update the service status every 30 seconds.
    pub async fn init() {
        let settings = match settings::ScoutQuestConfig::new() {
            Ok(settings) => settings,
            Err(e) => panic!("Error loading settings: {}", e)
        };
        let ip_addr = match local_ip() {
            Ok(ip_addr) => ip_addr,
            Err(e) => panic!("Error getting local ip address: {}", e)
        };
        let hostname = match gethostname().into_string() {
            Ok(hostname) => hostname,
            Err(e) => panic!("Error getting hostname: {:?}", e)
        };
        println!("{:?}", settings);

        let client = reqwest::Client::new();
        let map = serde_json::json!({
            "name": settings.scout_quest_config.service_name.replace(" ", "_").to_uppercase(),
            "ip_addr": ip_addr,
            "hostname": hostname,
            "port": settings.server.port
        });
        let url = format!("{}/api/services", settings.scout_quest_config.uri);
        match client.post(url)
            .json(&map)
            .send()
            .await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<ServiceResponse>().await {
                        Ok(resp) => {
                            unsafe {
                                UUID = Some(resp.uuid.clone());
                            };
                        },
                        Err(e) => panic!("Error parsing response: {}", e)
                    };
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

    /// Update the service status
    /// 
    /// # Panics
    /// This function will panic if the settings can not be loaded, the UUID can not be retrieved, the service status can not be updated.
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