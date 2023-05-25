
use crate::graphs;


#[get("/piegraph?<height>&<width>&<background>&<datapoint>&<style>&<teams>")]
pub fn piegraph(height: Option<i64>, width: Option<i64>, background: Option<String>, datapoint: Option<String>, style: Option<String>, teams: Option<String>) -> rocket::response::content::RawHtml<String> { // Style will let you switch between types of pie graphs
    let mut rdr = csv::Reader::from_path("data.csv").unwrap();
    // let mut i = 0.0;
    let binding = rdr.headers();
    let headers: Vec<String> = binding.unwrap().iter().map(|point| point.to_string()).collect();
    let mut teams_vec: Vec<String> = Vec::new();

    if teams.is_some() {
        teams_vec = teams.clone().unwrap().split(",").map(|s| s.parse().unwrap()).collect()
    }

    let mut team_header_position: usize = 0;
    if headers.contains(&"team".to_string()) {
        team_header_position = headers.iter().position(|r| r == &"team".to_string()).unwrap();
    }

    let mut header_position: usize = 0;
    if datapoint.is_none() {
        header_position = team_header_position.clone()
    }
    else{
        if headers.contains(&datapoint.clone().unwrap()) {
            header_position = headers.iter().position(|r| r == &datapoint.clone().unwrap()).unwrap()
        }
    }
    
    //                  ADD THE THING TO NARROW DOWN BETWEEN SPECIFIC TEAMS

    let mut graph = graphs::PieGraph::new(datapoint.unwrap_or("Team Numbers".to_string()), "#7289da".into());
    
    // If the user chooses specific teams
    if teams.is_some() {

        // Filters the records by team
        let records: Vec<Result<csv::StringRecord, csv::Error>> = rdr.records().filter(|s|
            {
                s.as_ref().unwrap().iter().any(|m| {
                    teams_vec.contains(&m.trim().replace("\"", "").to_string())
                })
            }
        ).collect();

        
        for result in records {
    
            let record = result.unwrap();
            graph.add_slice(
                record[header_position].to_string().replace("\"", "").trim().parse::<i32>().unwrap().into(), 
                record[team_header_position].to_string().trim().to_string().replace("\"", "")
            );
        }
    }
    // If no teams were chosen, Chooses every single team in there
    else {
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we should probably do some error checking here
    
            let record = result.unwrap();
            graph.add_slice(record[header_position].to_string().replace("\"", "").trim().parse::<i32>().unwrap().into(), record[team_header_position].to_string().trim().to_string().replace("\"", ""));
        }
    }
    
    // std::fs::write("test.svg", graph.draw_svg(height.unwrap_or(800), width.unwrap_or(1000), background.clone().unwrap_or("#1e1e2e".to_string())).unwrap());
    rocket::response::content::RawHtml(graph.draw_svg(height.unwrap_or(800), width.unwrap_or(1000), background.unwrap_or("#1e1e2e".to_string())).unwrap())
}

#[get("/linegraph?<height>&<width>&<datapoint>&<team>")]
pub fn linegraph(height: Option<usize>, width: Option<usize>, datapoint: Option<String>, team: Option<String>) -> rocket::response::content::RawXml<String> {
    let mut rdr = csv::Reader::from_path("data.csv").unwrap();
    let mut i = 0.0;
    let binding = rdr.headers();
    let headers: Vec<String> = binding.unwrap().iter().map(|point| point.to_string()).collect();

    let mut team_header_position: usize = 0;
    if headers.contains(&"team".to_string()) {
        team_header_position = headers.iter().position(|r| r == &"team".to_string()).unwrap();
    }

    let mut graph = graphs::LineGraph::new("yeah".into(), "#7289da".into());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.unwrap();
        graph.add_point(i, record[20].to_string().trim().parse::<i32>().unwrap().into());
        i += 1.0;
    }
    // graph.add_point(1.0, 1.0);
    // std::fs::write("test.svg", graph.draw_svg(1000, 800).unwrap());
    rocket::response::content::RawXml(graph.draw_svg(width.unwrap_or(1000), height.unwrap_or(800), 10).unwrap())
}