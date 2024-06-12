use actix_web::{delete, get, HttpResponse, post, Responder, web::Json};
use actix_web::web::{Data, Path};
use validator::Validate;

use crate::db::database::Database;
use crate::models::incoming::{AddOrUpdateUniqueIdentifierRequest, GetProductLocationsByCode, GetProductLocationsByName};
use crate::models::outgoing::RemoveUniqueIdentifierRequest;

//GET / unique identifiers
#[get("/unique_identifiers")]
async fn get_unique_identifiers(db: Data<Database>) -> impl Responder {
	match db.get_all_locations().await {
		Ok(found_locations) => {
			if found_locations.is_empty() {
				HttpResponse::NotFound().body("No data available in the database")
			} else {
				HttpResponse::Ok().json(found_locations)
			}
		}
		Err(_) => HttpResponse::InternalServerError().body("Error retrieving unique identifiers"),
	}
}

#[get("/unique_identifiers_name/{product_name}")]
async fn get_locations_by_product_name(
	db: Data<Database>,
	product_name: Path<GetProductLocationsByName>,
) -> impl Responder {
	let is_valid = product_name.validate();

	match is_valid {
		Ok(_) => match db.get_product_locations_by_name(&product_name).await {
			Ok(locations) => {
				if !locations.is_empty() {
					HttpResponse::Ok().json(locations)
				} else {
					HttpResponse::NotFound().body("No locations found for the specified product.")
				}
			}
			Err(_) => HttpResponse::InternalServerError().body("Failed to find the locations."),
		},
		Err(_) => HttpResponse::InternalServerError().body("Error retrieving unique identifiers"),
	}
}

#[get("/unique_identifiers_code/{product_code}")]
async fn get_locations_by_product_code(
	db: Data<Database>,
	product_code: Path<GetProductLocationsByCode>,
) -> impl Responder {
	let is_valid = product_code.validate();
	match is_valid {
		Ok(_) => match db.get_product_locations_by_code(&product_code).await {
			Ok(locations) => {
				if !locations.is_empty() {
					HttpResponse::Ok().json(locations)
				} else {
					HttpResponse::NotFound().body("No locations found for the specified product.")
				}
			}
			Err(_) => HttpResponse::InternalServerError().body("Failed to find the locations."),
		},
		Err(_) => HttpResponse::InternalServerError().body("Error retrieving unique identifiers"),
	}
}

#[post("/add_or_update_unique_identifier")]
async fn add_or_update_unique_identifier(
	db: Data<Database>,
	body: Json<AddOrUpdateUniqueIdentifierRequest>,
) -> impl Responder {
	let is_valid = body.validate();
	match is_valid {
		Ok(_) => match db.add_or_update_unique_identifier(&body).await {
			Ok(_) => HttpResponse::Ok().body("Identifier added or updated successfully!"),
			Err(_) => {
				HttpResponse::InternalServerError().body("Failed to add or update identifier")
			}
		},
		Err(_) => HttpResponse::BadRequest()
			.body("Invalid input. Please provide valid identifier details."),
	}
}

#[delete("/remove_unique_identifiers")]
async fn remove_unique_identifier(
	body: Json<RemoveUniqueIdentifierRequest>,
	db: Data<Database>,
) -> impl Responder {
	let is_valid = body.validate();

	match is_valid {
		Ok(_) => {
			match db.remove_unique_identifier(&body).await {
				Ok(_) => HttpResponse::Ok().body("Identifier updated or removed succefully!"),
				Err(_) => HttpResponse::InternalServerError().body("Faile to update or remove identifier. Posible reason: Not enough quantity for removal."),
			}
		}
		Err(_) => HttpResponse::BadRequest().body("Invalid input. Please provide valid identifier details.")
	}
}
