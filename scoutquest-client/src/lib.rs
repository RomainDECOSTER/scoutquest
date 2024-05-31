use serde::{Deserialize, Serialize};
mod settings;


#[derive(Deserialize, Serialize, Debug)]
struct ServiceResponse {
    uuid: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ServiceUrlResponse {
    url: String,
}

mod status {
    pub const UP: &str = "Up";
}

/// discovery_client module
/// 
/// This module is responsible for registering the service to the server and updating the service status.
/// 
/// # Example
/// ```
/// use scoutquest_client::discovery_client::init;
/// 
/// async fn main() {
///     init();
/// }
/// ```
/// # Note
/// This module uses the settings module to load the settings.
/// 
/// This module will start a schedul task to update the service status every 30 seconds.
/// 
/// This module will also register a signal handler to delete the service when the program is terminated.
/// 
/// # Panics
/// 
/// This module will panic if the settings can not be loaded, the local ip address can not be retrieved, the hostname can not be retrieved, the service can not be registered, the service status can not be updated, the scheduler can not be initialized, the job can not be created.
pub mod discovery_client {

    use std::{error::Error, thread};

    use crate::{status, ServiceResponse};
    use gethostname::gethostname;
    use local_ip_address::local_ip;
    use crate::settings;

    static mut UUID : Option<String> = None;

    /// Initialize the discovery client
    /// 
    /// # Panics
    /// This function will panic if the settings can not be loaded, the local ip address can not be retrieved, the hostname can not be retrieved, the service can not be registered, the service status can not be updated, the scheduler can not be initialized, the job can not be created.
    /// 
    /// # Note
    /// This function will start a scheduler to update the service status every 30 seconds.
    
    pub fn init() -> Result<(), Box<dyn Error>>{
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(30));
                match get_service() {
                    Ok(_) => {},
                    Err(e) => {
                        if e == "Service not found" {
                            let _ = register_service();
                        } else {
                            panic!("Error getting service: {}", e);
                        }
                    }
                };
                let _ = update_status(status::UP.to_string());
            }
        });
        register_service()?;
        let mut signals = signal_hook::iterator::Signals::new(&[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT])?;

        thread::spawn(move || {
            for _ in signals.forever() {
                let _ = delete_service();
                std::process::exit(0);
            }
        });
        Ok(())
    }

    /// Delete the service
    /// 
    /// # Panics
    /// 
    /// This function will panic if the settings can not be loaded, the UUID can not be retrieved, the service can not be deleted.
    fn delete_service() -> Result<(), Box<dyn Error>> {
        let settings = match settings::ScoutQuestConfig::new() {
            Ok(settings) => settings,
            Err(e) => panic!("Error loading settings: {}", e)
        };
        let client = reqwest::blocking::Client::new();
        let uuid = get_uuid();
        let url = format!("{}/api/services/{}", settings.scout_quest_config.uri, uuid);
        match client.delete(url)
            .send() {
            Ok(_) => Ok(()),
            Err(e) => {
                panic!("Error deleting service: {}", e);
            }
        }
    }

    /// Update the service status
    /// 
    /// # Parameters
    /// - status: String
    /// 
    /// # Panics
    /// This function will panic if the settings can not be loaded, the UUID can not be retrieved, the service status can not be updated.
    fn update_status(status: String) -> Result<(), Box<dyn Error>> {
        let settings = match settings::ScoutQuestConfig::new() {
            Ok(settings) => settings,
            Err(e) => panic!("Error loading settings: {}", e)
        };
        
        let client = reqwest::blocking::Client::new();
        let uuid = get_uuid();
        let url = format!("{}/api/services/{}?status={}", settings.scout_quest_config.uri, uuid, status);
        match client.put(url)
            .send() {
            Ok(_) => Ok(()),
            Err(e) => {
                panic!("Error updating service status: {}", e);
            }
        }
    }

    /// Get the service
    /// 
    /// # Panics
    /// 
    /// This function will panic if the settings can not be loaded, the UUID can not be retrieved, the service can not be retrieved.
    fn get_service() -> Result<(), String> {
        let settings = match settings::ScoutQuestConfig::new() {
            Ok(settings) => settings,
            Err(e) => panic!("Error loading settings: {}", e)
        };
        let client = reqwest::blocking::Client::new();
        let uuid = get_uuid();
        let url = format!("{}/api/services/{}", settings.scout_quest_config.uri, uuid);
        match client.get(url)
            .send() {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<ServiceResponse>() {
                        Ok(resp) => {
                            match uuid == resp.uuid {
                                true => Ok(()),
                                false => Err("UUID mismatch".into())
                            }
                        },
                        Err(e) => panic!("Error parsing response: {}", e)
                    }
                } else if resp.status().as_u16() == 404 {
                    Err("Service not found".into())
                } else {
                    Err("Error getting service".into())
                }
            
            },
            Err(e) => {
                panic!("Error registering service: {}", e);
            }
        }
    }

    /// Register the service
    /// 
    /// # Panics
    /// 
    /// This function will panic if the settings can not be loaded, the local ip address can not be retrieved, the hostname can not be retrieved, the service can not be registered.
    fn register_service() -> Result<(), Box<dyn Error>> {
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

        let client = reqwest::blocking::Client::new();
        let map = serde_json::json!({
            "name": settings.scout_quest_config.service_name.replace(" ", "_").to_uppercase(),
            "ip_addr": ip_addr,
            "hostname": hostname,
            "port": settings.server.port
        });
        let url = format!("{}/api/services", settings.scout_quest_config.uri);
        match client.post(url)
            .json(&map)
            .send() {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<ServiceResponse>() {
                        Ok(resp) => {
                            unsafe {
                                UUID = Some(resp.uuid.clone());
                            };
                        },
                        Err(e) => panic!("Error parsing response: {}", e)
                    };
                    Ok(())
                } else {
                    panic!("Error registering service: {}", resp.status());
                }
            
            },
            Err(e) => {
                panic!("Error registering service: {}", e);
            }
        }
    }

    /// Get the UUID
    /// 
    /// # Panics
    /// 
    /// This function will panic if the UUID is not set.
    fn get_uuid () -> String {
        match unsafe { UUID.clone() } {
            Some(uuid) => uuid,
            None => panic!("UUID not set")
        }
    }

    /// Discovery service
    /// 
    /// This function will call the service discovery service to get the service url.
    /// 
    /// # Example
    /// 
    /// ```
    /// use scoutquest_client::discovery_client::discovery_service;
    /// 
    /// fn main() {
    ///    discovery_service::call_service();
    /// }
    /// ```
    /// 
    /// # Note
    /// 
    /// This function must be call to get the service url before excuting an action on it.
    pub mod discovery_service {
        use crate::{settings, ServiceUrlResponse};
        use std::error::Error;

        /// Call the service
        /// 
        /// This function will retrieve the service name from the settings and call the service discovery service to get the service url.
        /// 
        /// # Panics
        /// 
        /// This function will panic if the settings can not be loaded, the service name can not be retrieved, the service url can not be retrieved.
        pub fn call_service() {
            let settings = match settings::ScoutQuestConfig::new() {
                Ok(settings) => settings,
                Err(e) => panic!("Error loading settings: {}", e)
            };
            println!("Calling service: {}", &settings.scout_quest_config.service_name.replace(" ", "_").to_uppercase());
            let url = match get_service_url(settings.scout_quest_config.service_name.replace(" ", "_").to_uppercase()) {
                Ok(url) => url,
                Err(e) => panic!("Error getting service url: {}", e)
            };
            println!("Service url: {}", url);
        }

        /// Get the service url
        /// 
        /// # Parameters
        /// - service_name: String
        /// 
        /// # Returns
        /// String - The service url
        /// 
        /// # Panics
        /// 
        /// This function will panic if the service url can not be retrieved.
        fn get_service_url(service_name: String) -> Result<String, Box<dyn Error>> {
            let settings = match crate::settings::ScoutQuestConfig::new() {
                Ok(settings) => settings,
                Err(e) => panic!("Error loading settings: {}", e)
            };
            let client = reqwest::blocking::Client::new();
            let map = serde_json::json!({
                "name": service_name
            });
            let url = format!("{}/api/services/url", settings.scout_quest_config.uri);
            match client.post(url)
                .json(&map)
                .send() {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<ServiceUrlResponse>() {
                            Ok(resp) => Ok(resp.url.into()),
                            Err(e) => panic!("Error parsing response: {}", e)
                        }
                    } else {
                        panic!("Error getting service url: {}", resp.status());
                    }
                
                },
                Err(e) => {
                    panic!("Error getting service url: {}", e);
                }
            }
        }
    }
}