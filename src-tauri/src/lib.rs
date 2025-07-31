use serde::{Deserialize, Serialize};
use std::error::Error;
use rust_xlsxwriter::*;

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
fn process_json(json_string: &str, selected: &str) -> Result<String, String> {//
    
    match save_to_excel(json_string, selected) {
        Ok(_) => Ok("Excel file created successfully!".to_string()),
        Err(e) => Err(format!("Error creating Excel file: {}", e)),
    }
}

fn save_to_excel(json_string: &str, selected: &str) -> Result<String, Box<dyn Error>> {
    // Parse the JSON string
    println!("Selected: {}", selected);
    let route_data: Vec<RouteData> = serde_json::from_str(json_string)?;
    println!("Parsed {} routes", route_data.len());

    let mut workbook = Workbook::new();
    
    // Create formats
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0xE6E6FA))
        .set_border(FormatBorder::Thin);
    
    let number_format = Format::new().set_num_format("#,##0.00");
    let integer_format = Format::new().set_num_format("#,##0");
    let percentage_format = Format::new().set_num_format("0.00%");

    // Add worksheet
    let worksheet = workbook.add_worksheet();

    // Define headers
    let headers = vec![
        "Airport ID", "Airport Name", "Full Name", "Country", "Continent", 
        "IATA", "ICAO", "Latitude", "Longitude", "Runway Length", "Market", "Hub Cost",
        "Runway Codes", "Pax Y Demand", "Pax J Demand", "Pax F Demand", 
        "Cargo L Demand", "Cargo H Demand", "Direct Distance", "Valid Route",
        "Needs Stopover", "Stopover Airport", "Stopover Country", "Flight Time",
        "Trips Per Day", "Aircraft Count", "Config Y", "Config J", "Config F",
        "Ticket Y", "Ticket J", "Ticket F", "Max Income", "Income", "Fuel Cost",
        "CO2 Emissions", "A-Check Cost", "Repair Cost", "Profit", "CI", "Contribution"
    ];

    // Write headers
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_with_format(0, col as u16, *header, &header_format)?;
        worksheet.set_column_width(col as u16, 15)?;
    }

    // Write data rows
    for (row_idx, route) in route_data.iter().enumerate() {
        let row = (row_idx + 1) as u32;
        
        // Airport data
        worksheet.write_with_format(row, 0, route.airport.id, &integer_format)?;
        worksheet.write(row, 1, &route.airport.name)?;
        worksheet.write(row, 2, &route.airport.fullname)?;
        worksheet.write(row, 3, &route.airport.country)?;
        worksheet.write(row, 4, &route.airport.continent)?;
        worksheet.write(row, 5, &route.airport.iata)?;
        worksheet.write(row, 6, &route.airport.icao)?;
        worksheet.write_with_format(row, 7, route.airport.lat, &number_format)?;
        worksheet.write_with_format(row, 8, route.airport.lng, &number_format)?;
        worksheet.write_with_format(row, 9, route.airport.rwy, &integer_format)?;
        worksheet.write_with_format(row, 10, route.airport.market, &integer_format)?;
        worksheet.write_with_format(row, 11, route.airport.hub_cost, &integer_format)?;
        worksheet.write(row, 12, &route.airport.rwy_codes)?;
        
        // Route demand data
        worksheet.write_with_format(row, 13, route.ac_route.route.pax_demand.y, &integer_format)?;
        worksheet.write_with_format(row, 14, route.ac_route.route.pax_demand.j, &integer_format)?;
        worksheet.write_with_format(row, 15, route.ac_route.route.pax_demand.f, &integer_format)?;
        worksheet.write_with_format(row, 16, route.ac_route.route.cargo_demand.l, &integer_format)?;
        worksheet.write_with_format(row, 17, route.ac_route.route.cargo_demand.h, &integer_format)?;
        worksheet.write_with_format(row, 18, route.ac_route.route.direct_distance, &number_format)?;
        
        // Route status
        worksheet.write(row, 19, if route.ac_route.valid { "Yes" } else { "No" })?;
        worksheet.write(row, 20, if route.ac_route.needs_stopover { "Yes" } else { "No" })?;
        
        // Stopover data
        if route.ac_route.needs_stopover {
            if let Some(ref stopover) = route.ac_route.stopover {
                if let Some(ref airport) = stopover.airport {
                    worksheet.write(row, 21, &airport.name)?;
                    worksheet.write(row, 22, &airport.country)?;
                } else {
                    worksheet.write(row, 21, "No stopover data")?;
                    worksheet.write(row, 22, "No stopover data")?;
                }
            } else {
                worksheet.write(row, 21, "No stopover data")?;
                worksheet.write(row, 22, "No stopover data")?;
            }
        } else {
            worksheet.write(row, 21, "N/A")?;
            worksheet.write(row, 22, "N/A")?;
        }
        
        // Flight data
        worksheet.write_with_format(row, 23, route.ac_route.flight_time, &number_format)?;
        worksheet.write_with_format(row, 24, route.ac_route.trips_per_day_per_ac, &integer_format)?;
        worksheet.write_with_format(row, 25, route.ac_route.num_ac, &integer_format)?;
        
        // Configuration
        worksheet.write_with_format(row, 26, route.ac_route.config.y, &integer_format)?;
        worksheet.write_with_format(row, 27, route.ac_route.config.j, &integer_format)?;
        worksheet.write_with_format(row, 28, route.ac_route.config.f, &integer_format)?;
        
        // Ticket prices
        worksheet.write_with_format(row, 29, route.ac_route.ticket.y, &integer_format)?;
        worksheet.write_with_format(row, 30, route.ac_route.ticket.j, &integer_format)?;
        worksheet.write_with_format(row, 31, route.ac_route.ticket.f, &integer_format)?;
        
        // Financial data
        worksheet.write_with_format(row, 32, route.ac_route.max_income, &number_format)?;
        worksheet.write_with_format(row, 33, route.ac_route.income, &number_format)?;
        worksheet.write_with_format(row, 34, route.ac_route.fuel, &number_format)?;
        worksheet.write_with_format(row, 35, route.ac_route.co2, &number_format)?;
        worksheet.write_with_format(row, 36, route.ac_route.acheck_cost, &number_format)?;
        worksheet.write_with_format(row, 37, route.ac_route.repair_cost, &number_format)?;
        worksheet.write_with_format(row, 38, route.ac_route.profit, &number_format)?;
        worksheet.write_with_format(row, 39, route.ac_route.ci, &integer_format)?;
        worksheet.write_with_format(row, 40, route.ac_route.contribution / 100.0, &percentage_format)?;
    }

    // Auto-fit some important columns
    worksheet.set_column_width(1, 20)?; // Airport name
    worksheet.set_column_width(2, 25)?; // Full name
    worksheet.set_column_width(3, 15)?; // Country
    worksheet.set_column_width(21, 20)?; // Stopover airport

    // Save the file to the user's Documents folder or current directory
    let filename = "route_analysis.xlsx";
    println!("Saving file: {}", filename);
    workbook.save(filename)?;
    println!("File saved successfully");

    Ok(filename.to_string())
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