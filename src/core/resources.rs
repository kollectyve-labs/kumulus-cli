use super::types::ResourceStatus;

pub fn list_resources() {
    println!("{:<21}{:<21}", "Resource Id", "Status");
    println!("{:-<36}", "");
    let resources = get_resources();

    match resources {
        Some(resources) => {
            resources.iter().for_each(|res| {
                println!("{:<21}{:<21?}", res.0, res.1);
            });
        }
        None => {
            println!("{:<21}{:<21}", "", "");
        }
    }
}

pub fn get_resources() -> Option<Vec<(String, ResourceStatus)>> {
    Some(vec![
        ("8dfec345".to_string(), ResourceStatus::Running),
        ("78cdf44h".to_string(), ResourceStatus::Running),
        ("26be8fk6".to_string(), ResourceStatus::Stopped)
    ])
}