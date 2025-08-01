use serde::{Deserialize, Serialize};
use std::error::Error;
use rust_xlsxwriter::*;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Airport {
    id: u32,
    name: String,
    fullname: String,
    country: String,
    continent: String,
    iata: String,
    icao: String,
    lat: f64,
    lng: f64,
    rwy: u32,
    market: u32,
    hub_cost: u32,
    rwy_codes: String,
}

#[derive(Deserialize, Debug)]
struct PaxDemand {
    y: u32,
    j: u32,
    f: u32,
}

#[derive(Deserialize, Debug)]
struct CargoDemand {
    l: u32,
    h: u32,
}

#[derive(Deserialize, Debug)]
struct Route {
    pax_demand: PaxDemand,
    cargo_demand: CargoDemand,
    direct_distance: f64,
}

#[derive(Deserialize, Debug)]
struct Stopover {
    airport: Option<Airport>,
    full_distance: Option<f64>,
    exists: bool,
}

#[derive(Deserialize, Debug)]
struct Config {
    y: u32,
    j: u32,
    f: u32,
    algorithm: String,
}

#[derive(Deserialize, Debug)]
struct Ticket {
    y: u32,
    j: u32,
    f: u32,
}

#[derive(Deserialize, Debug)]
struct AcRoute {
    route: Route,
    warnings: Vec<String>,
    valid: bool,
    max_tpd: Option<u32>,
    needs_stopover: bool,
    stopover: Option<Stopover>,
    flight_time: f64,
    trips_per_day_per_ac: u32,
    num_ac: u32,
    config: Config,
    ticket: Ticket,
    max_income: f64,
    income: f64,
    fuel: f64,
    co2: f64,
    acheck_cost: f64,
    repair_cost: f64,
    profit: f64,
    ci: u32,
    contribution: f64,
}

#[derive(Deserialize, Debug)]
struct RouteData {
    airport: Airport,
    ac_route: AcRoute,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn process_json(json_string: &str, selected: &str, mode: &str) -> Result<String, String> {//
    println!("Switch: {}", mode);
    match save_to_excel(json_string, selected, mode) {
        Ok(file_path) => Ok(file_path),
        Err(e) => Err(format!("Error creating Excel file: {}", e)),
    }
}

fn save_to_excel(json_string: &str, selected: &str, mode: &str) -> Result<String, Box<dyn Error>> {
    // Parse the JSON string
    
    println!("Selected: {}", selected);
    let route_data: Vec<RouteData> = serde_json::from_str(json_string)?;
    println!("Parsed {} routes", route_data.len());

    let mut workbook = Workbook::new();
    
    // Create formats
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x006600))
        .set_border(FormatBorder::Thin);
    let light_green = Format::new()
        .set_background_color(Color::RGB(0x99FF99));
    let light_grey = Format::new()
        .set_background_color(Color::RGB(0xC6C6C6));
    let green_grey_mix = Format::new()
        .set_background_color(Color::RGB(0xA1C5A1));
    let white = Format::new()
        .set_background_color(Color::RGB(0xFFFFFF));

    let number_format = Format::new().set_num_format("#,##0.00");
    let integer_format = Format::new().set_num_format("#,##0");
    let percentage_format = Format::new().set_num_format("0.00%");

    // Add worksheet
    let worksheet = workbook.add_worksheet();
    let choice = mode; // pax,cargo
    // Define headers
    let headers: Vec<&str>;
    if choice == "pax"{
        headers = vec![
            "ID", "Airport Name", "Full Name", "Country",  
            "Runway Length", "Config Y-J-F", "Ticket Y-J-F", "Demand Y-J-F",
            "Direct Distance", "Needs Stopover", "Stopover Airport", "Stopover Country",
            "Flight Time", "Trips Per Day", "Aircraft Count", "Max Income", "Income", "Fuel Cost",
            "CO2 Emissions", "A-Check Cost", "Repair Cost"
        ];
    }
    else if choice == "cargo"{
        headers = vec![
            "ID", "Airport Name", "Full Name", "Country",  
            "Runway Length", "Demand L", "Demand H",
            "Direct Distance", "Needs Stopover", "Stopover Airport", "Stopover Country",
            "Flight Time", "Trips Per Day", "Aircraft Count", "Max Income", "Income", "Fuel Cost",
            "CO2 Emissions", "A-Check Cost", "Repair Cost"
        ];
    }
    else {
        headers = vec![
            "Airport ID", "Airport Name", "Full Name", "Country", "Continent", 
            "IATA", "ICAO", "Latitude", "Longitude", "Runway Length", "Market", "Hub Cost",
            "Runway Codes", "Pax Y Demand", "Pax J Demand", "Pax F Demand", 
            "Cargo L Demand", "Cargo H Demand", "Direct Distance", "Valid Route",
            "Needs Stopover", "Stopover Airport", "Stopover Country", "Flight Time",
            "Trips Per Day", "Aircraft Count", "Config Y", "Config J", "Config F",
            "Ticket Y", "Ticket J", "Ticket F", "Max Income", "Income", "Fuel Cost",
            "CO2 Emissions", "A-Check Cost", "Repair Cost", "Profit", "CI", "Contribution"
        ];
    }
    // Write headers
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_with_format(0, col as u16, *header, &header_format)?;
        worksheet.set_column_width(col as u16, 15)?;
    }

    // Write data rows
    let mut color_one = light_grey.clone();
    let mut color_two = green_grey_mix.clone();
    let mut column = 0;
    for (row_idx, route) in route_data.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        column = 0;
        let (color_one, color_two) = if row % 2 == 0 {
            (&light_grey, &green_grey_mix)
        } else {
            (&white, &light_green)
        };
        // Airport data
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, row_idx as i64, format_to_use)?;
        column += 1; 
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, &route.airport.name, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, &route.airport.fullname, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, &route.airport.country, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.airport.rwy as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };

        //ticket price
        if choice == "pax"{
            worksheet.write_with_format(row, column, format!("{}-{}-{}", route.ac_route.config.y, route.ac_route.config.j, route.ac_route.config.f), format_to_use)?;
            column += 1;
            let format_to_use = if column % 2 == 0 { color_one } else { color_two };
            worksheet.write_with_format(row, column, format!("{}-{}-{}", route.ac_route.ticket.y, route.ac_route.ticket.j, route.ac_route.ticket.f), format_to_use)?;
            column += 1;
            let format_to_use = if column % 2 == 0 { color_one } else { color_two };
            worksheet.write_with_format(row, column, format!("{}-{}-{}", route.ac_route.route.pax_demand.y, route.ac_route.route.pax_demand.j, route.ac_route.route.pax_demand.f), format_to_use)?;
            column += 1;
        }
        else if choice == "cargo"{
            worksheet.write_with_format(row, column, route.ac_route.route.cargo_demand.l as f64, format_to_use)?;
            column += 1;
            let format_to_use = if column % 2 == 0 { color_one } else { color_two };
            worksheet.write_with_format(row, column, route.ac_route.route.cargo_demand.h as f64, format_to_use)?;
            column += 1;
        }

        // Distance
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.route.direct_distance as f64, format_to_use)?;
        column += 1;

        // Stop-over
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, if route.ac_route.needs_stopover { "Yes" } else { "No" }, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        if route.ac_route.needs_stopover {
            if let Some(ref stopover) = route.ac_route.stopover {
                if let Some(ref airport) = stopover.airport {
                    worksheet.write_with_format(row, column, &airport.name, format_to_use)?;
                    column += 1;
                    let format_to_use = if column % 2 == 0 { color_one } else { color_two };
                    worksheet.write_with_format(row, column, &airport.country, format_to_use)?;
                    column += 1;
                } else {
                    worksheet.write_with_format(row, column, "No stopover data", format_to_use)?;
                    column += 1;
                    let format_to_use = if column % 2 == 0 { color_one } else { color_two };
                    worksheet.write_with_format(row, column, "No stopover data", format_to_use)?;
                    column += 1;
                }
            } else {
                worksheet.write_with_format(row, column, "No stopover data", format_to_use)?;
                column += 1;
                let format_to_use = if column % 2 == 0 { color_one } else { color_two };
                worksheet.write_with_format(row, column, "No stopover data", format_to_use)?;
                column += 1;
            }
        } else {
            worksheet.write_with_format(row, column, "N/A", format_to_use)?;
            column += 1;
            let format_to_use = if column % 2 == 0 { color_one } else { color_two };
            worksheet.write_with_format(row, column, "N/A", format_to_use)?;
            column += 1;
        }
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.flight_time as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.trips_per_day_per_ac as i64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.num_ac as i64, format_to_use)?;
        column += 1;


        // Financial Data
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.max_income as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.income as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.fuel as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.co2 as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row, column, route.ac_route.acheck_cost as f64, format_to_use)?;
        column += 1;
        let format_to_use = if column % 2 == 0 { color_one } else { color_two };
        worksheet.write_with_format(row,column, route.ac_route.repair_cost as f64, format_to_use)?;
        


    }

    // Auto-fit some important columns
    
    worksheet.set_column_width(0, 5)?;  // ID
    worksheet.set_column_width(1, 25)?; // Airport name
    worksheet.set_column_width(2, 30)?; // Full name
    worksheet.set_column_width(3, 30)?; // Country
    worksheet.set_column_width(4, 8)?;  // Rnwy
    if choice == "pax"{
        worksheet.set_column_width(6, 20)?;  // ticket price
        worksheet.set_column_width(9, 5)?;  // needs stopover
        worksheet.set_column_width(10, 30)?;  // SO Airport name
        worksheet.set_column_width(11, 30)?;  // SO country
        worksheet.set_column_width(12, 10)?;  // Flight time 
        worksheet.set_column_width(13, 10)?;  // Trips a day
        worksheet.set_column_width(14, 10)?;  // AC no.
    }
    else if choice == "cargo"{

        worksheet.set_column_width(8, 5)?;  // needs stopover
        worksheet.set_column_width(9, 30)?;  // SO Airport name
        worksheet.set_column_width(10, 30)?;  // SO country
        worksheet.set_column_width(11, 10)?;  // Flight time 
        worksheet.set_column_width(12, 10)?;  // Trips a day
        worksheet.set_column_width(13, 10)?;  // AC no.
    }

    // Save the file to the user's Documents folder or current directory
    let filename = "route_analysis.xlsx";
    let path = Path::new(selected);
    
    let parent = path.parent().unwrap();
    let stem = path.file_stem().unwrap().to_string_lossy();
    let new_filename = format!("{}_{}.xlsx", stem, choice);
    let xlsx_path = parent.join(new_filename).to_string_lossy().to_string();
    println!("file stored in: {}", xlsx_path);
    workbook.save(&xlsx_path)?;
    println!("File saved successfully");

    Ok(xlsx_path.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, process_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}