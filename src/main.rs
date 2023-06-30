use wolfline::WolfLine;

mod api;
mod wolfline;

const API_URL: &'static str = "transloc-api-1-2.p.rapidapi.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tlm = WolfLine::new(include_str!("api-key"))?;

    tlm.find_ncsu().await?;
    tlm.find_route("30").await?;
    tlm.find_stop("village").await?;
    dbg!(tlm.get_time_remaining().await?);

    Ok(())
}
