use error_chain::error_chain;

error_chain !{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let res = reqwest::get("http://httpbin.org/get").await?;    //will wait as it is a asynchronous 
    println!("This is the Status : {}",res.status());
    println!("This is the Header : \n {:#?}",res.headers());

    let body = res.text().await?;
    println!("This is the Body : {}",body);
    Ok(())
}
