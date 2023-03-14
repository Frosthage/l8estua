use std::collections::HashSet;
use std::error::Error;
use std::ops::Sub;
use rss::Channel;



pub async fn get_latest() -> Result<String, Box<dyn Error>> {
    let content = reqwest::get("https://cornucopia.se/feed/ ")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;

    let ukraineWarItems: HashSet<&str> = vec!["försvar", "krig", "Ryssland", "Ukraina"].into_iter().collect();

    let urlItem = channel
        .items()
        .iter()
        .filter(|x| ukraineWarItems.is_subset(&x.categories.iter()
            .map(|x| x.name.as_str())
            .collect::<HashSet<&str>>()))
        .next();

    let result = urlItem.unwrap().link.as_ref().unwrap().clone();

    Ok(result)
}


#[tokio::test]
async fn test_something_async() {
    let result = get_latest().await;
    assert!(result.is_ok());
}

