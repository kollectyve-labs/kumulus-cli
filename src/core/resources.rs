use super::types::ResourceStatus;

pub fn list_resources() {
    println!("{:<21}{:<21}", "Resource Id", "Status");
    println!("{:-<42}", "");

    let resources = get_resources();

    if resources.is_empty() {
        println!("No resources available.");
    } else {
        for (id, status) in &resources {
            println!("{:<21}{:<21?}", id, status);
        }
    }
}

pub fn get_resources() -> Vec<(String, ResourceStatus)> {
    vec![
        ("8dfec345".to_string(), ResourceStatus::Running),
        ("78cdf44h".to_string(), ResourceStatus::Running),
        ("26be8fk6".to_string(), ResourceStatus::Stopped),
    ]
}
